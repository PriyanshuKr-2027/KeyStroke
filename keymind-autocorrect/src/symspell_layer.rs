use symspell::{AsciiStringStrategy, SymSpell, SymSpellBuilder, Verbosity};

/// Embedded 82k English frequency dictionary loader.
const DEFAULT_DICTIONARY: &str = include_str!("../data/frequency_dictionary_en_82k.txt");

pub struct SymSpellEngine {
    symspell: SymSpell<AsciiStringStrategy>,
}

impl Default for SymSpellEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SymSpellEngine {
    pub fn new() -> Self {
        let mut symspell: SymSpell<AsciiStringStrategy> = SymSpellBuilder::default()
            .max_dictionary_edit_distance(2)
            .prefix_length(7)
            .build()
            .unwrap();

        // Load embedded dictionary lines: "word count"
        for line in DEFAULT_DICTIONARY.lines() {
            let line = line.trim();
            if !line.is_empty() {
                symspell.load_dictionary_line(line, 0, 1, " ");
            }
        }

        Self { symspell }
    }

    /// Gboard Probabilistic Autocorrect Check (4 Rules Enforced):
    /// 1. Word length <= 3 gate: Skip autocorrect entirely for short words (is, in, at, the, you, go, ok)
    /// 2. Exact match passthrough: If typed word exists in dictionary, NEVER replace it
    /// 3. Strict Distance == 1 gate: Only allow 1 character edit distance for auto-replace
    /// 4. 10x Frequency Ratio gate: Suggestion count must be >= 10x the typed word baseline count
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        let word_lower = word.to_lowercase();
        let char_len = word_lower.chars().count();

        // Rule 1: Skip autocorrect entirely for words <= 3 characters
        if char_len <= 3 {
            return None;
        }

        // Rule 3: Exact match passthrough default.
        // If the typed word exists in dictionary (frequency >= 1), NEVER replace it!
        let exact_matches = self.symspell.lookup(&word_lower, Verbosity::Top, 0);
        if !exact_matches.is_empty() {
            return None;
        }

        // Rule 2 & 4: Only allow edit distance == 1 for auto-replacement
        let suggestions = self.symspell.lookup(&word_lower, Verbosity::Closest, 1);
        if suggestions.is_empty() {
            return None;
        }

        let best = &suggestions[0];

        // Ensure strict distance == 1
        if best.distance != 1 {
            return None;
        }

        // Skip if suggestion matches typed word
        if best.term.eq_ignore_ascii_case(&word_lower) {
            return None;
        }

        // Frequency Ratio Gate: Suggestion count must be at least 10x typed word baseline (1)
        let typed_freq = exact_matches.first().map(|s| s.count).unwrap_or(1);
        if (best.count as f64) < (typed_freq as f64 * 10.0) {
            return None;
        }

        // Probabilistic confidence score (0.85 to 0.99)
        let base_confidence = 0.88f32;
        let freq_boost = ((best.count as f32).ln().max(0.0) / 25.0).min(0.10);
        let confidence = (base_confidence + freq_boost).min(0.99);

        Some((best.term.clone(), confidence))
    }
}
