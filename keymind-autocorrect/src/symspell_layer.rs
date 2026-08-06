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

    /// Check word against SymSpell dictionary with strict edit distance rules.
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        // Skip single and double letter words (e.g. "a", "i", "in", "it", "to", "is", "of", "on", "at")
        if word.chars().count() <= 2 {
            return None;
        }

        let word_lower = word.to_lowercase();
        let char_len = word_lower.chars().count();

        // 1. CRITICAL: If the typed word is ALREADY a valid English word in the dictionary,
        // NEVER auto-correct it to another word!
        let exact_matches = self.symspell.lookup(&word_lower, Verbosity::Top, 0);
        if !exact_matches.is_empty() {
            return None;
        }

        // 2. Strict Edit Distance: Short words (3-4 chars) allow max edit distance 1 only.
        let max_distance = if char_len <= 4 { 1 } else { 2 };

        let suggestions = self.symspell.lookup(&word_lower, Verbosity::Closest, max_distance);
        if suggestions.is_empty() {
            return None;
        }

        let best = &suggestions[0];

        // Skip if suggestion matches typed word
        if best.term.eq_ignore_ascii_case(&word_lower) {
            return None;
        }

        // Calculate confidence score normalized to [0.70, 0.99]
        let base_confidence = match best.distance {
            1 => 0.88f32,
            2 => 0.75f32,
            _ => 0.50f32,
        };

        let freq_boost = ((best.count as f32).ln().max(0.0) / 25.0).min(0.10);
        let confidence = (base_confidence + freq_boost).min(0.99);

        Some((best.term.clone(), confidence))
    }
}
