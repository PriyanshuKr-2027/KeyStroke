use parking_lot::RwLock;
use std::collections::HashSet;

/// In-memory personal dictionary for sub-millisecond O(1) lookups.
pub struct PersonalDictionary {
    words: RwLock<HashSet<String>>,
}

impl PersonalDictionary {
    pub fn new(initial_words: HashSet<String>) -> Self {
        Self {
            words: RwLock::new(initial_words),
        }
    }

    /// Check if word is present in personal dictionary (case-insensitive).
    pub fn contains(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        self.words.read().contains(&word_lower)
    }

    /// Add word to in-memory set.
    pub fn insert(&self, word: &str) {
        let word_lower = word.to_lowercase();
        self.words.write().insert(word_lower);
    }
}
