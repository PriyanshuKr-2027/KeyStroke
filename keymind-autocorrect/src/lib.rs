pub mod bigram;
pub mod db;
pub mod homophones;
pub mod learning;
pub mod personal;
pub mod predictor;
pub mod symspell_layer;

use bigram::BigramModel;
use db::{DbHandler, SqlitePool};
use homophones::HomophoneResolver;
use learning::LearnedCorrections;
use personal::PersonalDictionary;

pub use predictor::TrigramPredictor;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use symspell_layer::SymSpellEngine;

/// Represents a proposed autocorrect suggestion.
#[derive(Debug, Clone, PartialEq)]
pub struct Correction {
    pub original: String,
    pub corrected: String,
    pub confidence: f32,
}

/// Real-time multi-layered autocorrect engine.
pub struct AutocorrectEngine {
    db_handler: DbHandler,
    personal_dict: PersonalDictionary,
    learned_corrections: LearnedCorrections,
    homophone_resolver: HomophoneResolver,
    symspell_engine: SymSpellEngine,
    bigram_model: BigramModel,
}

impl AutocorrectEngine {
    /// Construct a new AutocorrectEngine connected to a SQLite connection pool.
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self {
            db_handler: DbHandler::new(db),
            personal_dict: PersonalDictionary::new(HashSet::new()),
            learned_corrections: LearnedCorrections::new(HashMap::new()),
            homophone_resolver: HomophoneResolver::new(),
            symspell_engine: SymSpellEngine::new(),
            bigram_model: BigramModel::new(),
        }
    }

    /// Construct engine manually with pre-populated in-memory caches.
    pub fn with_caches(
        db: Arc<SqlitePool>,
        personal_words: HashSet<String>,
        learned_map: HashMap<String, String>,
    ) -> Self {
        Self {
            db_handler: DbHandler::new(db),
            personal_dict: PersonalDictionary::new(personal_words),
            learned_corrections: LearnedCorrections::new(learned_map),
            homophone_resolver: HomophoneResolver::new(),
            symspell_engine: SymSpellEngine::new(),
            bigram_model: BigramModel::new(),
        }
    }

    /// Asynchronously initialize database tables and populate in-memory caches.
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        self.db_handler.init_db().await?;

        let personal = self.db_handler.load_personal_words().await?;
        for word in personal {
            self.personal_dict.insert(&word);
        }

        let learned = self.db_handler.load_learned_corrections().await?;
        for (from, to) in learned {
            self.learned_corrections.record(&from, &to, 3);
        }

        Ok(())
    }

    /// Evaluates `word` in context across correction layers with Bigram Context Re-ranking.
    pub fn check(&self, word: &str, context: &str) -> Option<Correction> {
        if word.trim().is_empty() {
            return None;
        }

        // Layer 1 — Personal Dictionary
        if self.personal_dict.contains(word) {
            return None;
        }

        // Layer 1.5 — User Learned Corrections
        if let Some(learned_to) = self.learned_corrections.get(word) {
            return Some(Correction {
                original: word.to_string(),
                corrected: learned_to,
                confidence: 1.0,
            });
        }

        // Layer 3 — Homophone Resolution via context pattern rules
        if let Some((homophone_to, confidence)) = self.homophone_resolver.resolve(word, context) {
            return Some(Correction {
                original: word.to_string(),
                corrected: homophone_to.to_string(),
                confidence,
            });
        }

        // Extract previous word from context
        let prev_word = context.split_whitespace().last().unwrap_or("");

        // Layer 2 — SymSpell Edit Distance with Bigram Context Re-ranking
        if let Some((suggested, base_confidence)) = self.symspell_engine.check(word) {
            let mut final_confidence = base_confidence;

            // Re-rank confidence if bigram context matches
            if !prev_word.is_empty() {
                if let Some(bg_score) = self.bigram_model.score(prev_word, &suggested) {
                    final_confidence = (base_confidence * 0.6 + bg_score * 0.4).min(0.99);
                }
            }

            return Some(Correction {
                original: word.to_string(),
                corrected: suggested,
                confidence: final_confidence,
            });
        }

        None
    }

    pub fn add_to_personal_dict(&self, word: &str) {
        self.personal_dict.insert(word);

        let word_owned = word.to_string();
        let db_handler = DbHandler::new(Arc::clone(&self.db_handler.pool));

        tokio::spawn(async move {
            if let Err(e) = db_handler.insert_personal_word(&word_owned).await {
                tracing::error!("Failed to persist to database: {}", e);
            }
        });
    }

    pub fn record_user_correction(&self, from: &str, to: &str) {
        let from_owned = from.to_string();
        let to_owned = to.to_string();
        let db_handler = DbHandler::new(Arc::clone(&self.db_handler.pool));
        let learned_corrections = self.learned_corrections.clone();

        tokio::spawn(async move {
            match db_handler.record_correction(&from_owned, &to_owned).await {
                Ok(count) => {
                    if count >= 3 {
                        learned_corrections.record(&from_owned, &to_owned, count);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to persist to database: {}", e);
                }
            }
        });
    }

    pub fn record_user_correction_in_memory(&self, from: &str, to: &str, count: i64) {
        self.learned_corrections.record(from, to, count);
    }
}
