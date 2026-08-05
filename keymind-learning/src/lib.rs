pub mod db;
pub mod ngram;
pub mod privacy;

pub use db::{
    add_app_to_blocklist, delete_phrase, get_learned_phrases, ignore_phrase, init_learning_tables,
    pin_phrase, LearnedPhrase,
};
pub use ngram::{CandidatePhrase, NgramExtractor};
pub use privacy::PrivacyFilter;

use parking_lot::RwLock;
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Debug, Clone)]
pub struct TypingEvent {
    pub text: String,
    pub app_id: Option<String>,
    pub is_sensitive: bool,
}

#[derive(Debug, Clone)]
pub enum LearningEvent {
    PhraseLearned(LearnedPhrase),
}

pub struct LearningEngine {
    db: Arc<SqlitePool>,
    privacy: Arc<RwLock<PrivacyFilter>>,
    enabled: Arc<AtomicBool>,
}

impl LearningEngine {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self {
            db,
            privacy: Arc::new(RwLock::new(PrivacyFilter::new())),
            enabled: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn toggle_learning(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn start(
        db: Arc<SqlitePool>,
        mut rx: mpsc::Receiver<TypingEvent>,
    ) -> JoinHandle<()> {
        let privacy = PrivacyFilter::new();
        let enabled = Arc::new(AtomicBool::new(true));

        tokio::spawn(async move {
            let _ = init_learning_tables(&db).await;
            let mut extractor = NgramExtractor::default();

            while let Some(evt) = rx.recv().await {
                if !enabled.load(Ordering::SeqCst) {
                    continue;
                }

                // Apply privacy filters
                if !privacy.is_safe(&evt.text, evt.app_id.as_deref(), evt.is_sensitive) {
                    continue;
                }

                // Process words through n-gram sliding window
                for word in evt.text.split_whitespace() {
                    let candidates = extractor.push_word(word);
                    for candidate in candidates {
                        if let Ok(Some(learned)) =
                            db::upsert_candidate(&db, &candidate.display_text).await
                        {
                            info!("Phrase promoted to learned memory: {}", learned.phrase);
                        }
                    }
                }
            }
        })
    }
}
