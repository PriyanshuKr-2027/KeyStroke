pub mod ai;
pub mod db;
pub mod dynamic;
pub mod trigger;

use ai::{get_ai_system_prompt, AiError, GroqClient, GroqClientTrait};
use db::{DbHandler, VarType, Variable};
use dynamic::DynamicResolver;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use trigger::{TriggerAction, TriggerDetector};

#[derive(Error, Debug)]
pub enum VariableError {
    #[error("Database IO error: {0}")]
    Db(#[from] std::io::Error),
    #[error("AI error: {0}")]
    Ai(#[from] AiError),
    #[error("Variable not found: {0}")]
    NotFound(String),
}

/// Represents an expansion task returned when a variable trigger matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpansionTask {
    Static {
        backspace_count: usize,
        replacement: String,
    },
    Dynamic {
        backspace_count: usize,
        replacement: String,
    },
    Ai {
        key: String,
        backspace_count: usize,
        system_prompt: String,
    },
}

/// Core variable resolution engine.
pub struct VariableEngine {
    db_handler: DbHandler,
    static_cache: RwLock<HashMap<String, String>>,
    trigger_detector: RwLock<TriggerDetector>,
    ai_client: Arc<dyn GroqClientTrait>,
}

impl VariableEngine {
    pub fn new(store_path: PathBuf) -> Self {
        let ai_client: Arc<dyn GroqClientTrait> = match GroqClient::new() {
            Ok(c) => Arc::new(c),
            Err(_) => {
                // Fallback client if API key missing at creation
                struct NoOpClient;
                #[async_trait::async_trait]
                impl GroqClientTrait for NoOpClient {
                    async fn generate(&self, _: &str, _: &str) -> Result<String, AiError> {
                        Err(AiError::MissingApiKey)
                    }
                }
                Arc::new(NoOpClient)
            }
        };

        Self {
            db_handler: DbHandler::new(store_path),
            static_cache: RwLock::new(HashMap::new()),
            trigger_detector: RwLock::new(TriggerDetector::new()),
            ai_client,
        }
    }

    /// Construct engine with custom/mock Groq AI client.
    pub fn with_ai_client(store_path: PathBuf, client: Arc<dyn GroqClientTrait>) -> Self {
        Self {
            db_handler: DbHandler::new(store_path),
            static_cache: RwLock::new(HashMap::new()),
            trigger_detector: RwLock::new(TriggerDetector::new()),
            ai_client: client,
        }
    }

    /// Initialize database schema and load static variables into fast in-memory cache.
    pub async fn initialize(&self) -> Result<(), VariableError> {
        self.db_handler.init_db().await?;

        let vars = self.db_handler.list_all().await?;
        let mut cache = self.static_cache.write();
        for v in vars {
            if v.var_type == VarType::Static {
                if let Some(val) = v.value {
                    cache.insert(v.key.to_lowercase(), val);
                }
            }
        }

        Ok(())
    }

    /// Process a keystroke character and return an ExpansionTask if a variable trigger matches.
    pub fn process_keystroke(&self, c: char) -> Option<ExpansionTask> {
        let action = self.trigger_detector.write().process_char(c);

        if let TriggerAction::Expand { key, backspace_count } = action {
            let key_lower = key.to_lowercase();

            // 1. Static variable check (< 1ms, synchronous O(1))
            if let Some(val) = self.resolve_static(&key_lower) {
                return Some(ExpansionTask::Static {
                    backspace_count,
                    replacement: val,
                });
            }

            // 2. Dynamic variable check
            if let Some(val) = DynamicResolver::resolve(&key_lower) {
                return Some(ExpansionTask::Dynamic {
                    backspace_count,
                    replacement: val,
                });
            }

            // 3. AI variable check
            if let Some(prompt) = Some(get_ai_system_prompt(&key_lower)) {
                return Some(ExpansionTask::Ai {
                    key: key_lower,
                    backspace_count,
                    system_prompt: prompt.to_string(),
                });
            }
        }

        None
    }

    /// Synchronous O(1) resolution for static variables.
    pub fn resolve_static(&self, key: &str) -> Option<String> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        self.static_cache.read().get(&key_clean).cloned()
    }

    /// Asynchronous resolution for AI variables via Groq client.
    pub async fn resolve_ai(&self, key: &str, clipboard: &str) -> Result<String, VariableError> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        let prompt = match Some(get_ai_system_prompt(&key_clean)) {
            Some(p) => p.to_string(),
            None => {
                let var_opt = self.db_handler.get(&key_clean).await?;
                var_opt
                    .and_then(|v| v.ai_prompt)
                    .ok_or_else(|| VariableError::NotFound(key.to_string()))?
            }
        };

        let response = self.ai_client.generate(&prompt, clipboard).await?;
        Ok(response)
    }

    /// Upsert variable into database and update static cache if applicable.
    pub async fn upsert(&self, v: Variable) -> Result<(), VariableError> {
        let key_clean = v.key.trim_start_matches('/').to_lowercase();

        if v.var_type == VarType::Static {
            if let Some(ref val) = v.value {
                self.static_cache.write().insert(key_clean.clone(), val.clone());
            }
        }

        self.db_handler.upsert(v).await?;
        Ok(())
    }

    /// Delete variable from database and static cache.
    pub async fn delete(&self, key: &str) -> Result<(), VariableError> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        self.static_cache.write().remove(&key_clean);
        self.db_handler.delete(&key_clean).await?;
        Ok(())
    }

    /// List all variables from database.
    pub async fn list_all(&self) -> Result<Vec<Variable>, VariableError> {
        let vars = self.db_handler.list_all().await?;
        Ok(vars)
    }
}
