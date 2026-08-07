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

/// Returns true if typed and candidate differ by exactly one QWERTY-adjacent
/// substitution (equal-length strings only).
fn is_qwerty_typo(typed: &str, candidate: &str) -> bool {
    let t: Vec<char> = typed.chars().collect();
    let c: Vec<char> = candidate.chars().collect();

    if t.len() != c.len() {
        return false;
    }

    let mut diff_count = 0;
    let mut t_diff = ' ';
    let mut c_diff = ' ';

    for (a, b) in t.iter().zip(c.iter()) {
        if a != b {
            diff_count += 1;
            t_diff = *a;
            c_diff = *b;
        }
    }

    diff_count == 1 && is_qwerty_adjacent(t_diff, c_diff)
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
        // FIX: max_dictionary_edit_distance(1) instead of (2).
        //
        // wolfgarbe (algorithm author) recommends distance=2 for *batch* spellcheck
        // but distance=1 for *real-time typing autocorrect* where false positives
        // (e.g. "issue" → "tissue") must be minimised.
        //
        // Distance=2 pre-computes all 2-edit neighbours in the delete index, which
        // means "issue" (distance 2 from "tissue") would be considered as a candidate.
        // Distance=1 closes that door entirely.
        let mut symspell: SymSpell<AsciiStringStrategy> = SymSpellBuilder::default()
            .max_dictionary_edit_distance(1)
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

    /// Three-layer autocorrect check:
    ///
    /// **Layer 0 — Google-10k whitelist (O(1) hash lookup)**
    /// The 10,000 most common English words by Google n-gram frequency are
    /// embedded as a static `HashSet`. If the typed word is in this set it is
    /// _always_ passed through unchanged, no matter what SymSpell would suggest.
    /// This single gate prevents every "issue → tissue", "grammar → gamer" class
    /// of false positive where a common word is mangled because a rarer-but-higher-
    /// frequency-in-the-82k-list word exists at edit distance 1.
    ///
    /// **Layer 1 — Short-word gate**
    /// Words ≤ 3 characters are skipped (is, in, at, go, ok, the).
    ///
    /// **Layer 2 — SymSpell distance=1 + QWERTY spatial threshold**
    /// Single unified `Verbosity::Closest` lookup at max distance=1.
    /// If `best.distance == 0` the typed word exists in the 82k dictionary →
    /// passthrough. If `best.distance == 1` we apply the QWERTY adjacency
    /// frequency multiplier before deciding whether to correct.
    pub fn check(&self, word: &str) -> Option<(String, f32)> {
        let word_lower = word.to_lowercase();
        let char_len = word_lower.chars().count();

        // ── Layer 0: Google-10k whitelist ──────────────────────────────────
        // Common English words must never be auto-corrected, full stop.
        if google_10k().contains(word_lower.as_str()) {
            return None;
        }

        // ── Layer 1: Short-word gate ───────────────────────────────────────
        if char_len <= 3 {
            return None;
        }

        // ── Layer 2: SymSpell distance=1 unified lookup ────────────────────
        // Verbosity::Closest at max_distance=1:
        //   • If word is in the dictionary  → best.distance == 0 → passthrough
        //   • If word is a 1-edit typo      → best.distance == 1 → evaluate
        //   • If word is completely unknown → empty vec            → passthrough
        let suggestions = self.symspell.lookup(&word_lower, Verbosity::Closest, 1);
        if suggestions.is_empty() {
            return None;
        }

        let best = &suggestions[0];

        // Exact match in 82k dictionary → valid word → never replace
        if best.distance == 0 {
            return None;
        }

        // Require strictly distance == 1 (redundant given init, but explicit)
        if best.distance != 1 {
            return None;
        }

        // Safety: skip if suggestion somehow equals typed word
        if best.term.eq_ignore_ascii_case(&word_lower) {
            return None;
        }

        // QWERTY spatial frequency threshold
        // Adjacent-key fat-finger typos need 2.5× the candidate's frequency to fire.
        // Non-adjacent (wrong-letter) typos need 10.0× — far stricter.
        let is_adjacent = is_qwerty_typo(&word_lower, &best.term);
        let freq_multiplier = if is_adjacent { 2.5f64 } else { 10.0f64 };

        // Typed word is not in the 82k dict (distance != 0) so its baseline freq = 1
        if (best.count as f64) < freq_multiplier {
            return None;
        }

        // Confidence score 0.85–0.99
        let base_confidence = if is_adjacent { 0.92f32 } else { 0.85f32 };
        let freq_boost = ((best.count as f32).ln().max(0.0) / 25.0).min(0.07);
        let confidence = (base_confidence + freq_boost).min(0.99);

        Some((best.term.clone(), confidence))
    }
}
