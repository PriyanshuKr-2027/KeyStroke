use parking_lot::RwLock;
use std::collections::HashMap;

/// In-memory cache for user learned corrections (Layer 1.5).
pub struct LearnedCorrections {
    corrections: RwLock<HashMap<String, String>>,
}

impl LearnedCorrections {
    pub fn new(initial_map: HashMap<String, String>) -> Self {
        Self {
            corrections: RwLock::new(initial_map),
        }
    }

    /// Retrieve auto-learned correction if threshold count >= 3 has been reached.
    pub fn get(&self, from_word: &str) -> Option<String> {
        let from_lower = from_word.to_lowercase();
        self.corrections.read().get(&from_lower).cloned()
    }

    /// Record new correction count; if count >= 3, add to in-memory map.
    pub fn record(&self, from_word: &str, to_word: &str, count: i64) {
        if count >= 3 {
            let from_lower = from_word.to_lowercase();
            self.corrections
                .write()
                .insert(from_lower, to_word.to_string());
        }
    }
}
