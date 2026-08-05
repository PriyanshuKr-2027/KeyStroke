use keymind_autocorrect::AutocorrectEngine;
use keymind_grammar::GrammarEngine;
use keymind_learning::{LearningEngine, TypingEvent};
use keymind_prediction::PredictionEngine;
use keymind_variables::VariableEngine;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

pub struct TypingPipeline {
    autocorrect: Arc<AutocorrectEngine>,
    variables: Arc<VariableEngine>,
    grammar: Arc<GrammarEngine>,
    prediction: Arc<PredictionEngine>,
    learning_tx: mpsc::Sender<TypingEvent>,
}

impl TypingPipeline {
    pub fn new(
        autocorrect: Arc<AutocorrectEngine>,
        variables: Arc<VariableEngine>,
        grammar: Arc<GrammarEngine>,
        prediction: Arc<PredictionEngine>,
        learning_tx: mpsc::Sender<TypingEvent>,
    ) -> Self {
        Self {
            autocorrect,
            variables,
            grammar,
            prediction,
            learning_tx,
        }
    }

    pub async fn process_word(&self, word: &str, context: &str, is_sensitive: bool) {
        if is_sensitive {
            info!("Sensitive input detected - bypassing pipeline");
            return;
        }

        // 1. Send event to learning engine
        let _ = self
            .learning_tx
            .send(TypingEvent {
                text: word.to_string(),
                app_id: None,
                is_sensitive: false,
            })
            .await;

        // 2. Check variable resolution
        if word.starts_with('/') {
            match self.variables.resolve_static(word) {
                Some(replacement) => {
                    info!("Variable resolved: {} -> {}", word, replacement);
                    return;
                }
                None => {
                    // Try dynamic resolution
                    if let Some(replacement) = keymind_variables::dynamic::DynamicResolver::resolve(word) {
                        info!("Dynamic variable resolved: {} -> {}", word, replacement);
                        return;
                    }
                }
            }
        }

        // 3. Autocorrect check
        if let Some(correction) = self.autocorrect.check(word, context) {
            info!("Autocorrect applied: {} -> {}", word, correction.corrected);
            return;
        }

        // 4. Next-word prediction query
        let (suggestions, conf) = self.prediction.predict(context).await;
        if let Some(top_word) = suggestions.first() {
            let active = keymind_prediction::ActivePrediction {
                candidate_word: top_word.clone(),
                full_suggestions: suggestions.clone(),
                confidence: conf,
                context: context.to_string(),
            };
            info!(
                "Prediction ready: '{}' from {:?} (confidence: {:.2})",
                active.candidate_word, active.full_suggestions, conf
            );
        }
    }
}
