use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPhrase {
    pub id: String,
    pub phrase: String,
    pub frequency: i32,
    pub is_pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub frequency: i32,
    pub first_seen: i64,
    pub last_seen: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LearningState {
    pub candidates: HashMap<String, Candidate>,
    pub learned: HashMap<String, LearnedPhrase>,
    pub blocklist: HashSet<String>,
}

#[derive(Clone)]
pub struct DbHandler {
    store_path: PathBuf,
    state: Arc<RwLock<LearningState>>,
}

impl DbHandler {
    pub fn new(store_path: PathBuf) -> Self {
        Self {
            store_path,
            state: Arc::new(RwLock::new(LearningState::default())),
        }
    }

    pub async fn init_db(&self) -> Result<(), std::io::Error> {
        if self.store_path.exists() {
            let data = tokio::fs::read_to_string(&self.store_path).await?;
            if let Ok(parsed) = serde_json::from_str(&data) {
                *self.state.write() = parsed;
            }
        }
        Ok(())
    }

    fn save_state(&self) {
        let state = self.state.read().clone();
        let path = self.store_path.clone();
        tokio::spawn(async move {
            if let Ok(json) = serde_json::to_string_pretty(&state) {
                let _ = tokio::fs::write(path, json).await;
            }
        });
    }

    pub async fn upsert_candidate(&self, phrase: &str) -> Result<Option<LearnedPhrase>, std::io::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut promoted = None;

        {
            let mut state = self.state.write();
            
            let cand = state.candidates.entry(phrase.to_string()).or_insert(Candidate {
                frequency: 0,
                first_seen: now,
                last_seen: now,
            });
            cand.frequency += 1;
            cand.last_seen = now;

            let freq = cand.frequency;
            let last_seen = cand.last_seen;
            let seven_days_ago = now - (7 * 86400);

            if (freq >= 3 && last_seen >= seven_days_ago) || freq >= 10 {
                // Promote
                let id = Uuid::new_v4().to_string();
                let phrase_owned = phrase.to_string();
                
                // If it already exists in learned, just update freq
                if let Some((_, existing)) = state.learned.iter_mut().find(|(_, p)| p.phrase == phrase_owned) {
                    existing.frequency = freq;
                } else {
                    let new_learned = LearnedPhrase {
                        id: id.clone(),
                        phrase: phrase_owned.clone(),
                        frequency: freq,
                        is_pinned: false,
                    };
                    state.learned.insert(id, new_learned.clone());
                    promoted = Some(new_learned);
                }
            }
        }

        self.save_state();
        Ok(promoted)
    }

    pub async fn prune_old_candidates(&self) -> Result<u64, std::io::Error> {
        let thirty_days_ago = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - (30 * 86400);

        let removed: u64;
        {
            let mut state = self.state.write();
            let keys_to_remove: Vec<String> = state.candidates.iter()
                .filter(|(_, c)| c.last_seen < thirty_days_ago && c.frequency < 2)
                .map(|(k, _)| k.clone())
                .collect();

            for k in &keys_to_remove {
                state.candidates.remove(k);
            }
            removed = keys_to_remove.len() as u64;
        }

        if removed > 0 {
            self.save_state();
        }

        Ok(removed)
    }

    pub async fn get_learned_phrases(&self) -> Result<Vec<LearnedPhrase>, std::io::Error> {
        let state = self.state.read();
        let mut phrases: Vec<_> = state.learned.values().cloned().collect();
        phrases.sort_by(|a, b| {
            b.is_pinned.cmp(&a.is_pinned).then(b.frequency.cmp(&a.frequency))
        });
        Ok(phrases)
    }

    pub async fn pin_phrase(&self, id: &str) -> Result<(), std::io::Error> {
        {
            let mut state = self.state.write();
            if let Some(phrase) = state.learned.get_mut(id) {
                phrase.is_pinned = true;
            }
        }
        self.save_state();
        Ok(())
    }

    pub async fn delete_phrase(&self, id: &str) -> Result<(), std::io::Error> {
        {
            let mut state = self.state.write();
            state.learned.remove(id);
        }
        self.save_state();
        Ok(())
    }

    pub async fn ignore_phrase(&self, id: &str) -> Result<(), std::io::Error> {
        self.delete_phrase(id).await
    }

    pub async fn add_app_to_blocklist(&self, app_id: &str) -> Result<(), std::io::Error> {
        {
            let mut state = self.state.write();
            state.blocklist.insert(app_id.to_string());
        }
        self.save_state();
        Ok(())
    }
}
