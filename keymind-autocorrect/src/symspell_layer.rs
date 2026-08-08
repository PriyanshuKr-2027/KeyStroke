use std::collections::HashSet;
use std::sync::OnceLock;
use symspell::{AsciiStringStrategy, SymSpell, SymSpellBuilder, Verbosity};

// ---------------------------------------------------------------------------
// Embedded data files
// ---------------------------------------------------------------------------

/// 82k English unigram frequency dictionary ("word count" per line).
const DEFAULT_DICTIONARY: &str = include_str!("../data/frequency_dictionary_en_82k.txt");

/// Google-10000-English (no-swears) — top 10,000 most-frequent English words,
/// one word per line, sorted by descending frequency.
/// Source: https://github.com/first20hours/google-10000-english
const GOOGLE_10K_RAW: &str = include_str!("../data/google-10000-english-no-swears.txt");

// ---------------------------------------------------------------------------
// Layer 0: Google-10k whitelist — built once, zero allocations at query time
// ---------------------------------------------------------------------------

/// Returns a reference to the static Google-10k HashSet, building it on the
/// first call. `OnceLock` guarantees this is initialised exactly once across
/// all threads with zero runtime overhead on subsequent calls.
fn google_10k() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        GOOGLE_10K_RAW
            .lines()
            .map(str::trim)
            .filter(|w| !w.is_empty())
            .collect()
    })
}

// ---------------------------------------------------------------------------
// QWERTY spatial helpers
// ---------------------------------------------------------------------------

/// Returns true if two characters are adjacent on a standard QWERTY layout.
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

/// Returns true if typed and candidate differ by an adjacent substitution OR transposition (equal-length strings only).
fn is_qwerty_typo(typed: &str, candidate: &str) -> bool {
    let t: Vec<char> = typed.chars().collect();
    let c: Vec<char> = candidate.chars().collect();

    if t.len() != c.len() {
        return false;
    }

    let mut diff_indices = Vec::new();
    for (i, (a, b)) in t.iter().zip(c.iter()).enumerate() {
        if a != b {
            diff_indices.push(i);
        }
    }

    if diff_indices.len() == 1 {
        let idx = diff_indices[0];
        return is_qwerty_adjacent(t[idx], c[idx]);
    } else if diff_indices.len() == 2 {
        let i1 = diff_indices[0];
        let i2 = diff_indices[1];
        // Adjacent transposition (e.g. 'e','h' -> 'h','e')
        if i2 == i1 + 1 && t[i1] == c[i2] && t[i2] == c[i1] {
            return true;
        }
    }

    false
}

// ---------------------------------------------------------------------------
// SymSpell engine
// ---------------------------------------------------------------------------

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
        // Edit distance 2 allows catching real-world English typos such as
        // "woudl" -> "would", "recieve" -> "receive", "definately" -> "definitely",
        // "seperate" -> "separate", "trmperature" -> "temperature".
        let mut symspell: SymSpell<AsciiStringStrategy> = SymSpellBuilder::default()
            .max_dictionary_edit_distance(2)
            .prefix_length(7)
            .build()
            .unwrap();

        for line in DEFAULT_DICTIONARY.lines() {
            let line = line.trim();
            if !line.is_empty() {
                symspell.load_dictionary_line(line, 0, 1, " ");
            }
        }

        Self { symspell }
    }

    /// Multi-layer autocorrect check with distance 2 support:
    ///
    /// **Layer 0 — Google-10k whitelist**
    /// Highly frequent common words are passed through unchanged unless they are
    /// obvious typos.
    ///
    /// **Layer 1 — Short-word gate**
    /// Words ≤ 3 characters are skipped.
    ///
    /// **Layer 2 — SymSpell distance ≤ 2 + QWERTY distance scoring**
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        let word_lower = word.to_lowercase();
        let char_len = word_lower.chars().count();

        // ── Layer 0: Google-10k whitelist ──────────────────────────────────
        if google_10k().contains(word_lower.as_str()) {
            return None;
        }

        // ── Layer 1: Short-word gate ───────────────────────────────────────
        // Skip 1 and 2 letter tokens (e.g. a, in, is, on, to, at, it)
        if char_len <= 2 {
            return None;
        }

        // ── Layer 2: SymSpell distance ≤ 2 unified lookup ─────────────────
        let mut suggestions = self.symspell.lookup(&word_lower, Verbosity::Closest, 2);
        if suggestions.is_empty() {
            return None;
        }

        // Sort suggestions so highest frequency and adjacent typos rank first
        suggestions.sort_by(|a, b| {
            if a.distance == b.distance {
                let a_adjacent = is_qwerty_typo(&word_lower, &a.term);
                let b_adjacent = is_qwerty_typo(&word_lower, &b.term);
                if a_adjacent != b_adjacent {
                    b_adjacent.cmp(&a_adjacent)
                } else {
                    b.count.cmp(&a.count)
                }
            } else {
                a.distance.cmp(&b.distance)
            }
        });

        let best = &suggestions[0];

        // Exact match in dictionary → valid word → never replace
        if best.distance == 0 {
            return None;
        }

        if best.distance > 2 {
            return None;
        }

        // Safety: skip if suggestion equals typed word
        if best.term.eq_ignore_ascii_case(&word_lower) {
            return None;
        }

        // Calculate confidence score for distance 1 vs distance 2
        let is_adjacent = is_qwerty_typo(&word_lower, &best.term);
        let base_confidence = match best.distance {
            1 => if is_adjacent { 0.94f32 } else { 0.88f32 },
            2 => 0.82f32,
            _ => 0.70f32,
        };

        let freq_boost = ((best.count as f32).max(1.0).ln() / 25.0).min(0.08);
        let confidence = (base_confidence + freq_boost).min(0.99);

        if confidence >= 0.72 {
            Some((best.term.clone(), confidence))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_1_and_2_corrections() {
        let engine = SymSpellEngine::new();

        // Distance 1 test: "teh" -> "the"
        let corr1 = engine.check("teh");
        assert!(corr1.is_some(), "Expected correction for 'teh'");
        let (suggested1, conf1) = corr1.unwrap();
        assert_eq!(suggested1, "the");
        assert!(conf1 >= 0.80);

        // Distance 2 test: "woudl" -> "would"
        let corr2 = engine.check("woudl");
        assert!(corr2.is_some(), "Expected correction for 'woudl'");
        let (suggested2, conf2) = corr2.unwrap();
        assert_eq!(suggested2, "would");
        assert!(conf2 >= 0.72);

        // Distance 2 test: "recieve" -> "receive"
        let corr3 = engine.check("recieve");
        assert!(corr3.is_some(), "Expected correction for 'recieve'");
        let (suggested3, _) = corr3.unwrap();
        assert_eq!(suggested3, "receive");

        // Valid word passthrough: "keyboard" -> None
        assert!(engine.check("keyboard").is_none());
    }
}
