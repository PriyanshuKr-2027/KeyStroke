pub mod onnx;
pub mod trigram;

pub use onnx::OnnxPredictor;
pub use trigram::TrigramEngine;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    trigram: TrigramEngine,
    onnx: OnnxPredictor,
}

impl PredictionEngine {
    pub async fn new(store_path: PathBuf, model_path: PathBuf) -> Result<Self, std::io::Error> {
        let trigram = TrigramEngine::new(store_path);
        trigram.init_db().await?;
        let _ = trigram.load_bundled_trigrams(BUNDLED_TRIGRAMS).await;

        Ok(Self {
            trigram,
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

            if let Ok((suggestions, confidence)) = self.trigram.query_trigrams(w1, w2).await {
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
        let _ = self.trigram.update_trigram(w1, w2, w3).await;
    }
}
