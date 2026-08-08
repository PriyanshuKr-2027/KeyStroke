use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrigramState {
    pub trigrams: HashMap<String, i64>,
}

#[derive(Clone)]
pub struct TrigramEngine {
    file_path: PathBuf,
    state: Arc<RwLock<TrigramState>>,
}

impl TrigramEngine {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            state: Arc::new(RwLock::new(TrigramState::default())),
        }
    }

    pub async fn init_db(&self) -> Result<(), std::io::Error> {
        if self.file_path.exists() {
            let data = fs::read_to_string(&self.file_path).await?;
            if let Ok(parsed) = serde_json::from_str(&data) {
                *self.state.write().await = parsed;
            }
        } else {
            self.save().await?;
        }
        Ok(())
    }

    async fn save(&self) -> Result<(), std::io::Error> {
        let state = self.state.read().await;
        let data = serde_json::to_string(&*state)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&self.file_path, data).await?;
        Ok(())
    }

    pub async fn load_bundled_trigrams(&self, tsv_content: &str) -> Result<usize, std::io::Error> {
        let mut state = self.state.write().await;
        let mut count = 0;

        for line in tsv_content.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                let w1 = parts[0].trim().to_lowercase();
                let w2 = parts[1].trim().to_lowercase();
                let w3 = parts[2].trim().to_lowercase();
                let cnt: i64 = parts[3].trim().parse().unwrap_or(1);

                let key = format!("{}|{}|{}", w1, w2, w3);
                state.trigrams.entry(key).or_insert(cnt);
                count += 1;
            }
        }

        drop(state);
        self.save().await?;
        info!("Preloaded {} trigrams into JSON store", count);
        Ok(count)
    }

    pub async fn query_trigrams(&self, w1: &str, w2: &str) -> Result<(Vec<String>, f32), std::io::Error> {
        let w1_clean = w1.trim().to_lowercase();
        let w2_clean = w2.trim().to_lowercase();

        let state = self.state.read().await;
        let mut matches = Vec::new();

        // Very basic query logic scanning the keys since it's a small dataset.
        // In a real trie, this would be optimized, but for < 10k bundles it's fast enough.
        for (key, count) in &state.trigrams {
            let parts: Vec<&str> = key.split('|').collect();
            if parts.len() == 3 {
                let p1 = parts[0];
                let p2 = parts[1];
                let p3 = parts[2];

                if p1 == w1_clean && p2 == w2_clean {
                    matches.push((p3.to_string(), *count));
                }
            }
        }

        // Fallbacks
        if matches.is_empty() && !w2_clean.is_empty() {
            for (key, count) in &state.trigrams {
                let parts: Vec<&str> = key.split('|').collect();
                if parts.len() == 3 && parts[1] == w2_clean {
                    matches.push((parts[2].to_string(), *count));
                }
            }
        }

        if matches.is_empty() && !w1_clean.is_empty() {
            for (key, count) in &state.trigrams {
                let parts: Vec<&str> = key.split('|').collect();
                if parts.len() == 3 && parts[1] == w1_clean {
                    matches.push((parts[2].to_string(), *count));
                }
            }
        }

        if matches.is_empty() {
            return Ok((Vec::new(), 0.0));
        }

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches.truncate(3);

        let total: i64 = matches.iter().map(|(_, c)| c).sum();
        let top_count = matches[0].1;
        let confidence = if total > 0 {
            ((top_count as f32) / (total as f32)).min(0.99)
        } else {
            0.0
        };

        if confidence < 0.30 {
            return Ok((Vec::new(), confidence));
        }

        let suggestions: Vec<String> = matches.into_iter().map(|(w3, _)| w3).collect();
        Ok((suggestions, confidence))
    }

    pub async fn update_trigram(&self, w1: &str, w2: &str, w3: &str) -> Result<(), std::io::Error> {
        let w1_clean = w1.trim().to_lowercase();
        let w2_clean = w2.trim().to_lowercase();
        let w3_clean = w3.trim().to_lowercase();

        if w1_clean.is_empty() || w2_clean.is_empty() || w3_clean.is_empty() {
            return Ok(());
        }

        let key = format!("{}|{}|{}", w1_clean, w2_clean, w3_clean);
        
        let mut state = self.state.write().await;
        let count = state.trigrams.entry(key).or_insert(0);
        *count += 1;
        
        drop(state);
        // Fire and forget save for performance, or we can await it
        let _ = self.save().await;

        Ok(())
    }
}
