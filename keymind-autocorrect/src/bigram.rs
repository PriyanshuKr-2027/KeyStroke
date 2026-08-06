use rustc_hash::FxHashMap;

const DEFAULT_BIGRAMS_BIN: &[u8] = include_bytes!("../data/bigrams_en.bin");

pub struct BigramModel {
    // (prev_word, word) -> score
    pair_scores: FxHashMap<(String, String), f32>,
}

impl Default for BigramModel {
    fn default() -> Self {
        Self::new()
    }
}

impl BigramModel {
    pub fn new() -> Self {
        let mut pair_scores = FxHashMap::default();
        let mut cursor = 0;
        let bytes = DEFAULT_BIGRAMS_BIN;

        if bytes.len() >= 4 {
            let count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
            cursor += 4;

            for _ in 0..count {
                if cursor >= bytes.len() {
                    break;
                }
                let len1 = bytes[cursor] as usize;
                cursor += 1;
                if cursor + len1 > bytes.len() {
                    break;
                }
                let w1 = String::from_utf8_lossy(&bytes[cursor..cursor + len1]).to_lowercase();
                cursor += len1;

                if cursor >= bytes.len() {
                    break;
                }
                let len2 = bytes[cursor] as usize;
                cursor += 1;
                if cursor + len2 > bytes.len() {
                    break;
                }
                let w2 = String::from_utf8_lossy(&bytes[cursor..cursor + len2]).to_lowercase();
                cursor += len2;

                if cursor + 4 > bytes.len() {
                    break;
                }
                let score = f32::from_le_bytes([
                    bytes[cursor],
                    bytes[cursor + 1],
                    bytes[cursor + 2],
                    bytes[cursor + 3],
                ]);
                cursor += 4;

                pair_scores.insert((w1, w2), score);
            }
        }

        Self { pair_scores }
    }

    /// Evaluates probability P(candidate | prev_word)
    pub fn score(&self, prev_word: &str, candidate: &str) -> Option<f32> {
        let prev = prev_word.to_lowercase();
        let cand = candidate.to_lowercase();

        self.pair_scores.get(&(prev, cand)).copied()
    }
}
