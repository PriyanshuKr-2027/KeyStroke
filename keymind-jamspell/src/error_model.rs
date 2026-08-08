use serde::{Deserialize, Serialize};

/// The Error Model (EM) calculates the probability that the user meant
/// to type the `target_word` when they actually typed the `typo_word`.
/// 
/// P(typo | target) is estimated using a weighted edit distance
/// based on keyboard layout or simple insertion/deletion/substitution penalties.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorModel {
    pub insert_penalty: f64,
    pub delete_penalty: f64,
    pub substitute_penalty: f64,
    pub transpose_penalty: f64,
}

impl Default for ErrorModel {
    fn default() -> Self {
        Self {
            // These are log-probabilities, so more negative = less likely.
            // A 0.0 penalty would mean 100% probability.
            insert_penalty: -3.0,
            delete_penalty: -3.0,
            substitute_penalty: -2.5, // slightly more likely due to fat-fingers
            transpose_penalty: -2.0,  // common fast-typing typo
        }
    }
}

impl ErrorModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate log P(typo | target_word)
    /// This uses a modified Levenshtein algorithm that adds log-probabilities 
    /// instead of integer distances.
    pub fn score_typo(&self, typo: &str, target: &str) -> f64 {
        if typo == target {
            return 0.0; // log(1.0) = 0.0 (Perfect match)
        }

        let typo_chars: Vec<char> = typo.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        let n = typo_chars.len();
        let m = target_chars.len();

        if n == 0 {
            return (m as f64) * self.delete_penalty;
        }
        if m == 0 {
            return (n as f64) * self.insert_penalty;
        }

        // DP table for log probabilities. 
        // We initialize with extreme negative values (-infinity).
        let mut dp = vec![vec![-1e9; m + 1]; n + 1];
        
        dp[0][0] = 0.0;
        
        for i in 1..=n {
            dp[i][0] = dp[i-1][0] + self.insert_penalty;
        }
        for j in 1..=m {
            dp[0][j] = dp[0][j-1] + self.delete_penalty;
        }

        for i in 1..=n {
            for j in 1..=m {
                let mut best = dp[i-1][j] + self.insert_penalty; // deletion from target (insertion in typo)
                
                let del_score = dp[i][j-1] + self.delete_penalty; // deletion from typo (insertion in target)
                if del_score > best {
                    best = del_score;
                }

                let match_score = if typo_chars[i-1] == target_chars[j-1] {
                    dp[i-1][j-1] // no penalty
                } else {
                    dp[i-1][j-1] + self.substitute_penalty
                };
                if match_score > best {
                    best = match_score;
                }

                // Transposition (Damerau-Levenshtein)
                if i > 1 && j > 1 
                   && typo_chars[i-1] == target_chars[j-2] 
                   && typo_chars[i-2] == target_chars[j-1] 
                {
                    let trans_score = dp[i-2][j-2] + self.transpose_penalty;
                    if trans_score > best {
                        best = trans_score;
                    }
                }

                dp[i][j] = best;
            }
        }

        dp[n][m]
    }
}
