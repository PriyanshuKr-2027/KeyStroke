use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

/// The Language Model (LM) is responsible for calculating the probability
/// of a word sequence occurring in natural language.
/// 
/// We use a Trigram model with Stupid Backoff.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguageModel {
    /// Maps a string word to a unique u32 ID to save memory.
    pub word_to_id: FxHashMap<String, u32>,
    /// Maps a u32 ID back to a string word.
    pub id_to_word: FxHashMap<u32, String>,
    
    /// Unigram counts: word_id -> count
    pub unigrams: FxHashMap<u32, u64>,
    /// Bigram counts: (word1_id, word2_id) -> count
    pub bigrams: FxHashMap<(u32, u32), u64>,
    /// Trigram counts: (word1_id, word2_id, word3_id) -> count
    pub trigrams: FxHashMap<(u32, u32, u32), u64>,

    pub total_words: u64,
}

impl LanguageModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve or create an ID for a word.
    pub fn get_or_create_word_id(&mut self, word: &str) -> u32 {
        if let Some(&id) = self.word_to_id.get(word) {
            return id;
        }
        let id = self.word_to_id.len() as u32;
        self.word_to_id.insert(word.to_string(), id);
        self.id_to_word.insert(id, word.to_string());
        id
    }

    pub fn get_word_id(&self, word: &str) -> Option<u32> {
        self.word_to_id.get(word).copied()
    }

    /// Add a sentence to the model to train the N-grams.
    pub fn train_sentence(&mut self, words: &[&str]) {
        let mut ids = Vec::with_capacity(words.len());
        for &w in words {
            ids.push(self.get_or_create_word_id(w));
        }

        for i in 0..ids.len() {
            // Unigram
            let w1 = ids[i];
            *self.unigrams.entry(w1).or_insert(0) += 1;
            self.total_words += 1;

            // Bigram
            if i >= 1 {
                let w0 = ids[i - 1];
                *self.bigrams.entry((w0, w1)).or_insert(0) += 1;
            }

            // Trigram
            if i >= 2 {
                let w_minus_2 = ids[i - 2];
                let w_minus_1 = ids[i - 1];
                *self.trigrams.entry((w_minus_2, w_minus_1, w1)).or_insert(0) += 1;
            }
        }
    }

    /// Calculate log-probability of a word given its context (previous 2 words).
    /// Uses Stupid Backoff: P(w3 | w1, w2) = count(w1, w2, w3) / count(w1, w2)
    /// If trigram not found, backs off to 0.4 * P(w3 | w2).
    /// If bigram not found, backs off to 0.4 * P(w3).
    pub fn score_trigram(&self, w1_id: u32, w2_id: u32, w3_id: u32) -> f64 {
        let alpha = 0.4_f64; // Backoff penalty

        // Try Trigram
        let tri_count = self.trigrams.get(&(w1_id, w2_id, w3_id)).copied().unwrap_or(0);
        if tri_count > 0 {
            let bi_context_count = self.bigrams.get(&(w1_id, w2_id)).copied().unwrap_or(0);
            if bi_context_count > 0 {
                return (tri_count as f64 / bi_context_count as f64).ln();
            }
        }

        // Try Bigram
        let bi_count = self.bigrams.get(&(w2_id, w3_id)).copied().unwrap_or(0);
        if bi_count > 0 {
            let uni_context_count = self.unigrams.get(&w2_id).copied().unwrap_or(0);
            if uni_context_count > 0 {
                let prob = (bi_count as f64 / uni_context_count as f64) * alpha;
                return prob.ln();
            }
        }

        // Try Unigram
        let uni_count = self.unigrams.get(&w3_id).copied().unwrap_or(0);
        if uni_count > 0 {
            let prob = (uni_count as f64 / self.total_words as f64) * alpha * alpha;
            return prob.ln();
        }

        // Unknown Word Penalty
        let prob = (1.0 / (self.total_words as f64 + 1.0)) * alpha * alpha * alpha;
        prob.ln()
    }
}
