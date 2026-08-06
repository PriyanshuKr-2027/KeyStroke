use symspell::{AsciiStringStrategy, SymSpell, SymSpellBuilder, Verbosity};

/// Embedded 82k English frequency dictionary loader.
const DEFAULT_DICTIONARY: &str = include_str!("../data/frequency_dictionary_en_82k.txt");

/// Check if two characters are adjacent on a standard QWERTY keyboard layout.
fn is_qwerty_adjacent(c1: char, c2: char) -> bool {
    let c1 = c1.to_ascii_lowercase();
    let c2 = c2.to_ascii_lowercase();
    if c1 == c2 {
        return true;
    }

    match c1 {
        'q' => matches!(c2, 'w' | 'a' | '1' | '2'),
        'w' => matches!(c2, 'q' | 'e' | 'a' | 's' | '2' | '3'),
        'e' => matches!(c2, 'w' | 'r' | 's' | 'd' | '3' | '4'),
        'r' => matches!(c2, 'e' | 't' | 'd' | 'f' | '4' | '5'),
        't' => matches!(c2, 'r' | 'y' | 'f' | 'g' | '5' | '6'),
        'y' => matches!(c2, 't' | 'u' | 'g' | 'h' | '6' | '7'),
        'u' => matches!(c2, 'y' | 'i' | 'h' | 'j' | '7' | '8'),
        'i' => matches!(c2, 'u' | 'o' | 'j' | 'k' | '8' | '9'),
        'o' => matches!(c2, 'i' | 'p' | 'k' | 'l' | '9' | '0'),
        'p' => matches!(c2, 'o' | 'l' | '0' | '-'),
        'a' => matches!(c2, 'q' | 'w' | 's' | 'z'),
        's' => matches!(c2, 'w' | 'e' | 'a' | 'd' | 'z' | 'x'),
        'd' => matches!(c2, 'e' | 'r' | 's' | 'f' | 'x' | 'c'),
        'f' => matches!(c2, 'r' | 't' | 'd' | 'g' | 'c' | 'v'),
        'g' => matches!(c2, 't' | 'y' | 'f' | 'h' | 'v' | 'b'),
        'h' => matches!(c2, 'y' | 'u' | 'g' | 'j' | 'b' | 'n'),
        'j' => matches!(c2, 'u' | 'i' | 'h' | 'k' | 'n' | 'm'),
        'k' => matches!(c2, 'i' | 'o' | 'j' | 'l' | 'm'),
        'l' => matches!(c2, 'o' | 'p' | 'k'),
        'z' => matches!(c2, 'a' | 's' | 'x'),
        'x' => matches!(c2, 'z' | 's' | 'd' | 'c'),
        'c' => matches!(c2, 'x' | 'd' | 'f' | 'v'),
        'v' => matches!(c2, 'c' | 'f' | 'g' | 'b'),
        'b' => matches!(c2, 'v' | 'g' | 'h' | 'n'),
        'n' => matches!(c2, 'b' | 'h' | 'j' | 'm'),
        'm' => matches!(c2, 'n' | 'j' | 'k'),
        _ => false,
    }
}

/// Check if two single-character difference strings differ by QWERTY-adjacent key.
fn is_qwerty_typo(typed: &str, candidate: &str) -> bool {
    let t_chars: Vec<char> = typed.chars().collect();
    let c_chars: Vec<char> = candidate.chars().collect();

    // Equal length substitution typo (e.g. thiz vs this)
    if t_chars.len() == c_chars.len() {
        let mut diff_count = 0;
        let mut t_diff = ' ';
        let mut c_diff = ' ';

        for (a, b) in t_chars.iter().zip(c_chars.iter()) {
            if a != b {
                diff_count += 1;
                t_diff = *a;
                c_diff = *b;
            }
        }

        if diff_count == 1 {
            return is_qwerty_adjacent(t_diff, c_diff);
        }
    }

    false
}

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

    /// Gboard + QWERTY Spatial Error Probabilistic Check:
    /// 1. Word length <= 3 gate: Skip autocorrect entirely for short words (is, in, at, the, you, go, ok)
    /// 2. Exact match passthrough: If typed word exists in dictionary, NEVER replace it
    /// 3. Strict Distance == 1 gate: Only allow 1 character edit distance for auto-replace
    /// 4. Spatial QWERTY Error Model: Adjacent key typos require 2.5x frequency, non-adjacent require 10x
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        let word_lower = word.to_lowercase();
        let char_len = word_lower.chars().count();

        // Rule 1: Skip autocorrect entirely for words <= 3 characters
        if char_len <= 3 {
            return None;
        }

        // Rule 2: Exact match passthrough default.
        // If the typed word exists in dictionary (frequency >= 1), NEVER replace it!
        let exact_matches = self.symspell.lookup(&word_lower, Verbosity::Top, 0);
        if !exact_matches.is_empty() {
            return None;
        }

        // Rule 3: Only allow edit distance == 1 for auto-replacement
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

        // Rule 4: QWERTY Spatial Error Frequency Threshold
        // Adjacent key typos (fat-finger) use 2.5x threshold, non-adjacent use 10.0x threshold
        let is_adjacent = is_qwerty_typo(&word_lower, &best.term);
        let freq_multiplier = if is_adjacent { 2.5 } else { 10.0 };

        let typed_freq = exact_matches.first().map(|s| s.count).unwrap_or(1);
        if (best.count as f64) < (typed_freq as f64 * freq_multiplier) {
            return None;
        }

        // Probabilistic confidence score (0.85 to 0.99)
        let base_confidence = if is_adjacent { 0.92f32 } else { 0.85f32 };
        let freq_boost = ((best.count as f32).ln().max(0.0) / 25.0).min(0.07);
        let confidence = (base_confidence + freq_boost).min(0.99);

        Some((best.term.clone(), confidence))
    }
}
