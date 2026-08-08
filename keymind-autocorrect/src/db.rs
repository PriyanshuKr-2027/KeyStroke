use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutocorrectState {
    pub personal_words: HashSet<String>,
    pub corrections: HashMap<String, CorrectionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionData {
    pub count: i64,
    pub last_seen: i64,
}

#[derive(Clone)]
pub struct DbHandler {
    file_path: PathBuf,
    state: Arc<RwLock<AutocorrectState>>,
}

impl DbHandler {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            state: Arc::new(RwLock::new(AutocorrectState::default())),
        }
    }

    /// Initialize JSON flat-file storage
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
        let data = serde_json::to_string_pretty(&*state)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&self.file_path, data).await?;
        Ok(())
    }

    /// Load all personal words into a HashSet.
    pub async fn load_personal_words(&self) -> Result<HashSet<String>, std::io::Error> {
        let state = self.state.read().await;
        Ok(state.personal_words.iter().map(|w| w.to_lowercase()).collect())
    }

    /// Load user learned corrections where count >= 3 into a HashMap (from_word -> to_word).
    pub async fn load_learned_corrections(&self) -> Result<HashMap<String, String>, std::io::Error> {
        let state = self.state.read().await;
        let mut map = HashMap::new();
        for (from, to_data) in &state.corrections {
            if to_data.count >= 3 {
                // Key format is expected to be "from:to", but existing sqlx db had two columns.
                // Since this is key-value, we'll store key as "from_word|to_word"
                let parts: Vec<&str> = from.split('|').collect();
                if parts.len() == 2 {
                    map.insert(parts[0].to_lowercase(), parts[1].to_string());
                }
            }
        }
        Ok(map)
    }

    /// Insert or ignore word into personal_words table.
    pub async fn insert_personal_word(&self, word: &str) -> Result<(), std::io::Error> {
        let word_clean = word.to_lowercase();
        let mut state = self.state.write().await;
        if state.personal_words.insert(word_clean) {
            drop(state);
            self.save().await?;
        }
        Ok(())
    }

    /// Upsert user correction and return new count.
    pub async fn record_correction(&self, from: &str, to: &str) -> Result<i64, std::io::Error> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let key = format!("{}|{}", from.to_lowercase(), to);
        
        let mut state = self.state.write().await;
        let entry = state.corrections.entry(key).or_insert(CorrectionData {
            count: 0,
            last_seen: 0,
        });
        
        entry.count += 1;
        entry.last_seen = now;
        
        let new_count = entry.count;
        drop(state);
        self.save().await?;
        
        Ok(new_count)
    }
}
