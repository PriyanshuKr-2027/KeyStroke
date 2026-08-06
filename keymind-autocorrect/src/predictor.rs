use std::collections::HashMap;

const DEFAULT_NGRAMS_BIN: &[u8] = include_bytes!("../data/ngrams_en.bin");

pub struct TrigramPredictor {
    // (w1, w2) -> Vec<suggestion>
    trigrams: HashMap<(String, String), Vec<String>>,
    // w1 -> Vec<suggestion>
    bigrams: HashMap<String, Vec<String>>,
}

impl Default for TrigramPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl TrigramPredictor {
    pub fn new() -> Self {
        let mut trigrams = HashMap::new();
        let mut bigrams = HashMap::new();
        let bytes = DEFAULT_NGRAMS_BIN;
        let mut cursor = 0;

        if bytes.len() >= 4 {
            // 1. Read Trigrams section
            let tri_count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
            cursor += 4;

            for _ in 0..tri_count {
                if cursor >= bytes.len() {
                    break;
                }
                let len1 = bytes[cursor] as usize;
                cursor += 1;
                let w1 = String::from_utf8_lossy(&bytes[cursor..cursor + len1]).to_lowercase();
                cursor += len1;

                let len2 = bytes[cursor] as usize;
                cursor += 1;
                let w2 = String::from_utf8_lossy(&bytes[cursor..cursor + len2]).to_lowercase();
                cursor += len2;

                let sug_count = bytes[cursor] as usize;
                cursor += 1;

                let mut sugs = Vec::with_capacity(sug_count);
                for _ in 0..sug_count {
                    let len_s = bytes[cursor] as usize;
                    cursor += 1;
                    let s = String::from_utf8_lossy(&bytes[cursor..cursor + len_s]).to_string();
                    cursor += len_s;
                    sugs.push(s);
                }

                trigrams.insert((w1, w2), sugs);
            }

            // 2. Read Bigrams section
            if cursor + 4 <= bytes.len() {
                let bi_count = u32::from_le_bytes([
                    bytes[cursor],
                    bytes[cursor + 1],
                    bytes[cursor + 2],
                    bytes[cursor + 3],
                ]) as usize;
                cursor += 4;

                for _ in 0..bi_count {
                    if cursor >= bytes.len() {
                        break;
                    }
                    let len1 = bytes[cursor] as usize;
                    cursor += 1;
                    let w1 = String::from_utf8_lossy(&bytes[cursor..cursor + len1]).to_lowercase();
                    cursor += len1;

                    let sug_count = bytes[cursor] as usize;
                    cursor += 1;

                    let mut sugs = Vec::with_capacity(sug_count);
                    for _ in 0..sug_count {
                        let len_s = bytes[cursor] as usize;
                        cursor += 1;
                        let s = String::from_utf8_lossy(&bytes[cursor..cursor + len_s]).to_string();
                        cursor += len_s;
                        sugs.push(s);
                    }

                    bigrams.insert(w1, sugs);
                }
            }
        }

        Self { trigrams, bigrams }
    }

    /// Predict next word candidates based on context (Stupid Backoff).
    pub fn predict(&self, context: &str) -> Vec<String> {
        let words: Vec<&str> = context.split_whitespace().collect();
        if words.is_empty() {
            return Vec::new();
        }

        let len = words.len();
        let last1 = words[len - 1].to_lowercase();

        // 1. Try Trigram match if at least 2 context words available
        if len >= 2 {
            let last2 = words[len - 2].to_lowercase();
            if let Some(sugs) = self.trigrams.get(&(last2, last1.clone())) {
                if !sugs.is_empty() {
                    return sugs.clone();
                }
            }
        }

        // 2. Backoff to Bigram match
        if let Some(sugs) = self.bigrams.get(&last1) {
            return sugs.clone();
        }

        Vec::new()
    }
}
