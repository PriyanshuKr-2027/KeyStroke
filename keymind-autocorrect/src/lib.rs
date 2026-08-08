pub mod bigram;
pub mod db;
pub mod homophones;
pub mod learning;
pub mod personal;
pub mod predictor;
pub mod symspell_layer;
pub mod typo_map;

use keymind_jamspell::JamSpellEngine;
use db::DbHandler;
use homophones::HomophoneResolver;
use learning::LearnedCorrections;
use personal::PersonalDictionary;
pub use typo_map::ExplicitTypoMap;

pub use predictor::TrigramPredictor;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
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
    jamspell_engine: JamSpellEngine,
}

impl AutocorrectEngine {
    /// Construct a new AutocorrectEngine with JSON store.
    pub fn new(store_path: PathBuf) -> Self {
        Self {
            db_handler: DbHandler::new(store_path),
            personal_dict: PersonalDictionary::new(HashSet::new()),
            learned_corrections: LearnedCorrections::new(HashMap::new()),
            homophone_resolver: HomophoneResolver::new(),
            symspell_engine: SymSpellEngine::new(),
            jamspell_engine: JamSpellEngine::new(),
        }
    }

    /// Construct engine manually with pre-populated in-memory caches.
    pub fn with_caches(
        store_path: PathBuf,
        personal_words: HashSet<String>,
        learned_map: HashMap<String, String>,
    ) -> Self {
        Self {
            db_handler: DbHandler::new(store_path),
            personal_dict: PersonalDictionary::new(personal_words),
            learned_corrections: LearnedCorrections::new(learned_map),
            homophone_resolver: HomophoneResolver::new(),
            symspell_engine: SymSpellEngine::new(),
            jamspell_engine: JamSpellEngine::new(),
        }
    }

    /// Asynchronously initialize database tables and populate in-memory caches.
    pub async fn initialize(&self) -> Result<(), std::io::Error> {
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

    /// Evaluates `word` in context across correction layers with JamSpell Context Re-ranking.
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

        // Layer 0.5 — High-Frequency Explicit Typo Map (O(1) instant resolution)
        if let Some(explicit_to) = ExplicitTypoMap::get(word) {
            return Some(Correction {
                original: word.to_string(),
                corrected: explicit_to.to_string(),
                confidence: 0.99,
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

        // Extract previous words from context
        let words: Vec<&str> = context.split_whitespace().collect();
        let prev1 = if words.len() >= 1 { words[words.len() - 1] } else { "" };
        let prev2 = if words.len() >= 2 { words[words.len() - 2] } else { "" };

        // Layer 2 — SymSpell Edit Distance with JamSpell Context Re-ranking
        if let Some((suggested, base_confidence)) = self.symspell_engine.check(word) {
            // JamSpell Re-ranking (Bayesian Error + Trigram LM)
            let jamspell_score = self.jamspell_engine.score_candidate(&suggested, word, prev1, prev2);
            
            // Normalize the log probability back into a confidence boost.
            // Since it's negative log prob, higher (closer to 0) is better.
            let jamspell_boost = (jamspell_score.max(-10.0) + 10.0) / 10.0;
            
            // Blend SymSpell structural confidence with JamSpell contextual confidence
            let final_confidence = (base_confidence * 0.5 + jamspell_boost as f32 * 0.5).min(0.99);

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
        let db_handler = self.db_handler.clone();

        tokio::spawn(async move {
            if let Err(e) = db_handler.insert_personal_word(&word_owned).await {
                tracing::error!("Failed to persist to database: {}", e);
            }
        });
    }

    pub fn record_user_correction(&self, from: &str, to: &str) {
        let from_owned = from.to_string();
        let to_owned = to.to_string();
        let db_handler = self.db_handler.clone();
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
