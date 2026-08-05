pub mod onnx;
pub mod trigram;

pub use onnx::OnnxPredictor;
pub use trigram::{init_trigram_table, load_bundled_trigrams, query_trigrams, update_trigram};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::Arc;

pub const BUNDLED_TRIGRAMS: &str = include_str!("../data/trigrams.tsv");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivePrediction {
    pub candidate_word: String,
    pub full_suggestions: Vec<String>,
    pub confidence: f32,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionEvent {
    PredictionReady {
        prediction: ActivePrediction,
    },
    PredictionDismissed,
}

pub struct PredictionEngine {
    db: Arc<SqlitePool>,
    onnx: OnnxPredictor,
}

impl PredictionEngine {
    pub async fn new(db: Arc<SqlitePool>, model_path: PathBuf) -> Result<Self, sqlx::Error> {
        init_trigram_table(&db).await?;
        let _ = load_bundled_trigrams(&db, BUNDLED_TRIGRAMS).await;

        Ok(Self {
            db,
            onnx: OnnxPredictor::new(model_path),
        })
    }

    /// Predicts next words given previous context string.
    /// Uses Tier 1 Trigram (< 1ms). Falls back to Tier 2 ONNX if confidence < 0.4.
    pub async fn predict(&self, context: &str) -> (Vec<String>, f32) {
        let words: Vec<&str> = context.split_whitespace().collect();
        if words.len() >= 2 {
            let w1 = words[words.len() - 2];
            let w2 = words[words.len() - 1];

            if let Ok((suggestions, confidence)) = query_trigrams(&self.db, w1, w2).await {
                if confidence >= 0.4 && !suggestions.is_empty() {
                    return (suggestions, confidence);
                }
            }
        }

        // Tier 2 ONNX async fallback
        let onnx_suggestions = self.onnx.predict_next_words(context).await;
        (onnx_suggestions, 0.35)
    }

    pub async fn record_word_sequence(&self, w1: &str, w2: &str, w3: &str) {
        let _ = update_trigram(&self.db, w1, w2, w3).await;
    }
}
