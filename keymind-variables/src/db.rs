use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VarType {
    Static,
    Dynamic,
    Ai,
}

impl VarType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VarType::Static => "static",
            VarType::Dynamic => "dynamic",
            VarType::Ai => "ai",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dynamic" => VarType::Dynamic,
            "ai" => VarType::Ai,
            _ => VarType::Static,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub var_type: VarType,
    pub value: Option<String>,
    pub ai_prompt: Option<String>,
    pub use_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VariablesState {
    pub variables: HashMap<String, Variable>,
}

#[derive(Clone)]
pub struct DbHandler {
    file_path: PathBuf,
    state: Arc<RwLock<VariablesState>>,
}

impl DbHandler {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            state: Arc::new(RwLock::new(VariablesState::default())),
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
        let data = serde_json::to_string_pretty(&*state)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&self.file_path, data).await?;
        Ok(())
    }

    pub async fn upsert(&self, mut v: Variable) -> Result<(), std::io::Error> {
        let key_clean = v.key.trim_start_matches('/').to_lowercase();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        v.updated_at = now;
        if v.created_at == 0 {
            v.created_at = now;
        }

        self.state.write().await.variables.insert(key_clean, v);
        self.save().await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), std::io::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        if self.state.write().await.variables.remove(&key_clean).is_some() {
            self.save().await?;
        }
        Ok(())
    }

    pub async fn list_all(&self) -> Result<Vec<Variable>, std::io::Error> {
        let state = self.state.read().await;
        let mut vars: Vec<Variable> = state.variables.values().cloned().collect();
        // Sort to ensure stable output, optional but good
        vars.sort_by_key(|v| v.key.clone());
        Ok(vars)
    }

    pub async fn get(&self, key: &str) -> Result<Option<Variable>, std::io::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        let state = self.state.read().await;
        Ok(state.variables.get(&key_clean).cloned())
    }

    pub async fn increment_use_count(&self, key: &str) -> Result<(), std::io::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        let mut state = self.state.write().await;
        if let Some(v) = state.variables.get_mut(&key_clean) {
            v.use_count += 1;
        }
        drop(state);
        self.save().await?;
        Ok(())
    }
}
