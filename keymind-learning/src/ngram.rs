use lazy_static::lazy_static;
use std::collections::{HashSet, VecDeque};

lazy_static! {
    static ref font_stop_words: HashSet<&'static str> = {
        let mut set = HashSet::new();
        let words = [
            "a", "an", "the", "and", "or", "but", "if", "then", "else", "when", "at", "by", "for",
            "with", "about", "against", "between", "into", "through", "during", "before", "after",
            "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under",
            "again", "further", "then", "once", "here", "there", "where", "why", "how", "all",
            "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor",
            "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will",
            "just", "don", "should", "now", "is", "am", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "having", "do", "does", "did", "doing", "it", "its", "this",
            "that", "these", "those",
        ];
        for w in words {
            set.insert(w);
        }
        set
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandidatePhrase {
    pub display_text: String,
    pub normalized_key: String,
    pub n: usize,
}

pub struct NgramExtractor {
    word_window: VecDeque<String>,
    max_capacity: usize,
}

impl Default for NgramExtractor {
    fn default() -> Self {
        Self::new(8)
    }
}

impl NgramExtractor {
    pub fn new(capacity: usize) -> Self {
        Self {
            word_window: VecDeque::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    /// Clean punctuation surrounding a word
    pub fn clean_token(token: &str) -> String {
        token
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string()
    }

    /// Push a word into the sliding window and return newly extracted valid n-grams for n in [2, 3, 4, 5].
    pub fn push_word(&mut self, raw_word: &str) -> Vec<CandidatePhrase> {
        let cleaned = Self::clean_token(raw_word);
        if cleaned.is_empty() {
            return Vec::new();
        }

        if self.word_window.len() >= self.max_capacity {
            self.word_window.pop_front();
        }
        self.word_window.push_back(cleaned);

        let mut results = Vec::new();
        let len = self.word_window.len();

        for n in 2..=5 {
            if len >= n {
                let slice: Vec<&str> = self
                    .word_window
                    .iter()
                    .skip(len - n)
                    .map(|s| s.as_str())
                    .collect();

                // Check if phrase consists solely of stop words
                let is_all_stop_words = slice
                    .iter()
                    .all(|w| font_stop_words.contains(w.to_lowercase().as_str()));

                if !is_all_stop_words {
                    let display_text = slice.join(" ");
                    let normalized_key = display_text.to_lowercase();
                    results.push(CandidatePhrase {
                        display_text,
                        normalized_key,
                        n,
                    });
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram_extraction() {
        let mut extractor = NgramExtractor::new(8);
        extractor.push_word("Quarterly");
        extractor.push_word("financial");
        let candidates = extractor.push_word("results");

        assert!(!candidates.is_empty());
        assert!(candidates
            .iter()
            .any(|c| c.normalized_key == "quarterly financial results"));
    }

    #[test]
    fn test_stop_word_filter() {
        let mut extractor = NgramExtractor::new(8);
        extractor.push_word("is");
        let candidates = extractor.push_word("a");
        assert!(candidates.is_empty());
    }
}
