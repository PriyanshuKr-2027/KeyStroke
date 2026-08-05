use symspell::{AsciiStringStrategy, SymSpell, SymSpellBuilder, Verbosity};

/// Embedded 82k English frequency dictionary snippet / dictionary loader.
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

    /// Check word against SymSpell dictionary with max edit distance 2.
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        if word.chars().count() <= 1 {
            return None;
        }

        let word_lower = word.to_lowercase();
        let suggestions = self.symspell.lookup(&word_lower, Verbosity::Closest, 2);
        
        if suggestions.is_empty() {
            return None;
        }

        let best = &suggestions[0];

        // If the best suggestion matches the typed word exactly, no correction needed
        if best.term.eq_ignore_ascii_case(&word_lower) {
            return None;
        }

        // Frequency threshold check: skip if suggestion frequency < typed_word_frequency * 0.1
        let typed_freq = self.symspell.lookup(&word_lower, Verbosity::Top, 0)
            .first()
            .map(|s| s.count)
            .unwrap_or(1);

        if (best.count as f64) < (typed_freq as f64 * 0.1) {
            return None;
        }

        // Calculate confidence score normalized to [0.5, 0.99] based on distance and frequency
        let base_confidence = match best.distance {
            1 => 0.85f32,
            2 => 0.70f32,
            _ => 0.50f32,
        };

        let freq_boost = ((best.count as f32).ln().max(0.0) / 20.0).min(0.14);
        let confidence = (base_confidence + freq_boost).min(0.99);

        Some((best.term.clone(), confidence))
    }
}
