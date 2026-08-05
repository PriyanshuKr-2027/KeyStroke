use std::collections::HashSet;

/// Confusable word sets and bigram/trigram context pattern matcher (Layer 3).
pub struct HomophoneResolver {
    confusable_set: HashSet<&'static str>,
}

impl Default for HomophoneResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl HomophoneResolver {
    pub fn new() -> Self {
        let mut set = HashSet::new();
        for words in &[
            &["their", "there", "they're"][..],
            &["your", "you're"][..],
            &["its", "it's"][..],
            &["then", "than"][..],
            &["to", "too", "two"][..],
        ] {
            for &w in *words {
                set.insert(w);
            }
        }

        Self { confusable_set: set }
    }

    pub fn is_confusable(&self, word: &str) -> bool {
        self.confusable_set.contains(word.to_lowercase().as_str())
    }

    /// Resolve homophone based on context string (trailing 2-3 words).
    pub fn resolve(&self, word: &str, context: &str) -> Option<(&'static str, f32)> {
        let word_lower = word.to_lowercase();
        if !self.is_confusable(&word_lower) {
            return None;
        }

        let context_lower = context.to_lowercase();
        let words: Vec<&str> = context_lower
            .split_whitespace()
            .collect();

        if words.is_empty() {
            return None;
        }

        let len = words.len();
        let prev1 = words.get(len.wrapping_sub(1)).copied().unwrap_or("");
        let prev2 = if len >= 2 { words.get(len - 2).copied().unwrap_or("") } else { "" };

        let bigram = format!("{} _", prev1);
        let trigram = format!("{} {} _", prev2, prev1);

        // Pattern rules mapping context to target homophone
        let suggestion = match (trigram.as_str(), bigram.as_str()) {
            // (their, there, they're)
            (_, "over _") | (_, "out _") | (_, "up _") | (_, "down _") | (_, "in _") => Some("there"),
            (_, "of _") | (_, "for _") | (_, "with _") if word_lower == "there" || word_lower == "they're" => Some("their"),
            ("they say _", _) | ("think that _", _) => Some("they're"),

            // (your, you're)
            (_, "is _") | (_, "was _") | (_, "are _") if word_lower == "you're" => Some("your"),
            (_, "what _") | (_, "when _") | (_, "know _") if word_lower == "your" => Some("you're"),

            // (its, it's)
            (_, "lost _") | (_, "changed _") | (_, "has _") => Some("its"),
            (_, "think _") | (_, "know _") | (_, "said _") if word_lower == "its" => Some("it's"),

            // (then, than)
            (_, "more _") | (_, "less _") | (_, "better _") | (_, "worse _") | (_, "rather _") => Some("than"),
            (_, "back _") | (_, "and _") | (_, "since _") | (_, "until _") => Some("then"),

            // (to, too, two)
            (_, "going _") | (_, "want _") | (_, "need _") | (_, "used _") => Some("to"),
            (_, "me _") | (_, "much _") | (_, "far _") | (_, "late _") => Some("too"),
            (_, "one _") | (_, "or _") | (_, "the _") if word_lower == "to" || word_lower == "too" => Some("two"),

            _ => None,
        };

        suggestion.map(|s| (s, 0.95))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homophone_patterns() {
        let resolver = HomophoneResolver::new();
        let res = resolver.resolve("their", "going over ");
        assert_eq!(res, Some(("there", 0.95)));

        let res2 = resolver.resolve("then", "more ");
        assert_eq!(res2, Some(("than", 0.95)));
    }
}
