pub mod error_model;
pub mod lm;

pub use error_model::ErrorModel;
pub use lm::LanguageModel;

use serde::{Deserialize, Serialize};

/// The main JamSpell-like autocorrect engine that combines a 
/// Bayesian Error Model with a Trigram Language Model.
#[derive(Debug, Serialize, Deserialize)]
pub struct JamSpellEngine {
    pub lm: LanguageModel,
    pub em: ErrorModel,
}

impl JamSpellEngine {
    pub fn new() -> Self {
        Self {
            lm: LanguageModel::new(),
            em: ErrorModel::new(),
        }
    }

    pub fn train_sentence(&mut self, sentence: &str) {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        self.lm.train_sentence(&words);
    }

    /// Generates candidates for a single word.
    /// In a full implementation, this uses a Trie or SymSpell to return a list of words
    /// within edit distance 2. For now, we mock it by returning the exact word if known,
    /// or just returning a static list of possibilities.
    fn generate_candidates(&self, word: &str) -> Vec<String> {
        let c = vec![word.to_string()];
        
        // This is a naive mockup for testing. In reality, we'd use our SymSpell layer here 
        // to generate `c`. We'll inject that later.
        
        c
    }

    /// Correct a sentence using the Viterbi algorithm.
    /// It seeks to maximize: Sum[ log P(w_i | w_{i-2}, w_{i-1}) + log P(typo_i | w_i) ]
    pub fn correct_sentence(&self, sentence: &str) -> String {
        let tokens: Vec<&str> = sentence.split_whitespace().collect();
        if tokens.is_empty() {
            return sentence.to_string();
        }

        // DP table for Viterbi. 
        // We need to keep track of the probability of the sequence ending in (w_{i-1}, w_i).
        // Since storing all pairs is memory-intensive, we just use a Beam Search 
        // or a bounded Viterbi (keeping top K states per step).
        
        // Currently, as a placeholder logic until SymSpell candidates are piped in:
        // We'll just return the sentence unchanged.
        
        sentence.to_string()
    }

    /// Exposes a direct scoring function for a single word given its context.
    /// This is useful for the pipeline which evaluates words sequentially.
    pub fn score_candidate(&self, candidate: &str, typo: &str, prev1: &str, prev2: &str) -> f64 {
        let w3_id = self.lm.get_word_id(candidate).unwrap_or(u32::MAX);
        let w2_id = self.lm.get_word_id(prev1).unwrap_or(u32::MAX);
        let w1_id = self.lm.get_word_id(prev2).unwrap_or(u32::MAX);

        let lm_score = self.lm.score_trigram(w1_id, w2_id, w3_id);
        let em_score = self.em.score_typo(typo, candidate);

        lm_score + em_score
    }
}

impl Default for JamSpellEngine {
    fn default() -> Self {
        Self::new()
    }
}
