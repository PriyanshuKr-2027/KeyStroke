pub mod db;
pub mod ngram;
pub mod privacy;

pub use db::{DbHandler, LearnedPhrase};
pub use ngram::{CandidatePhrase, NgramExtractor};
pub use privacy::PrivacyFilter;

use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::path::PathBuf;
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

#[derive(Clone)]
pub struct LearningEngine {
    pub db: DbHandler,
    privacy: Arc<RwLock<PrivacyFilter>>,
    pub enabled: Arc<AtomicBool>,
}

impl LearningEngine {
    pub fn new(store_path: PathBuf) -> Self {
        Self {
            db: DbHandler::new(store_path),
            privacy: Arc::new(RwLock::new(PrivacyFilter::new())),
            enabled: Arc::new(AtomicBool::new(true)),
        }
    }

    pub async fn initialize(&self) -> Result<(), std::io::Error> {
        self.db.init_db().await?;
        Ok(())
    }

    pub fn toggle_learning(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn start(
        &self,
        mut rx: mpsc::Receiver<TypingEvent>,
    ) -> JoinHandle<()> {
        let enabled = self.enabled.clone();
        let db = self.db.clone();
        let privacy = self.privacy.clone();

        tokio::spawn(async move {
            let mut extractor = NgramExtractor::default();

            while let Some(evt) = rx.recv().await {
                if !enabled.load(Ordering::Relaxed) {
                    continue;
                }

                // Apply privacy filters
                if !privacy.read().is_safe(&evt.text, evt.app_id.as_deref(), evt.is_sensitive) {
                    continue;
                }

                // Process words through n-gram sliding window
                for word in evt.text.split_whitespace() {
                    let candidates = extractor.push_word(word);
                    for candidate in candidates {
                        if let Ok(Some(learned)) =
                            db.upsert_candidate(&candidate.display_text).await
                        {
                            info!("Phrase promoted to learned memory: {}", learned.phrase);
                        }
                    }
                }
            }
        })
    }
}
