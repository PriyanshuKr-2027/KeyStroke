use crate::protocol::{
    DailyStatsDto, IpcRequest, IpcResponse, LearnedPhraseDto, VariableDto,
};
use keymind_learning::LearningEngine;
use keymind_variables::VariableEngine;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
#[cfg(unix)]
use tokio::net::UnixListener;
use tracing::info;

pub struct IpcServer {
    variable_engine: Arc<VariableEngine>,
    learning_engine: Arc<LearningEngine>,
    socket_path: String,
    learning_enabled: Arc<AtomicBool>,
}

impl IpcServer {
    pub fn new(
        variable_engine: Arc<VariableEngine>,
        learning_engine: Arc<LearningEngine>,
        socket_path: Option<&str>,
        learning_enabled: Arc<AtomicBool>,
    ) -> Self {
        let path = socket_path
            .unwrap_or(if cfg!(windows) {
                "127.0.0.1:9123"
            } else {
                "/tmp/keymind.sock"
            })
            .to_string();
        Self {
            variable_engine,
            learning_engine,
            socket_path: path,
            learning_enabled,
        }
    }

    /// Process a single incoming IpcRequest against the local engines.
    pub async fn handle_request(&self, request: IpcRequest) -> IpcResponse {
        match request {
            IpcRequest::STATUS_REQUEST => IpcResponse::STATUS_RESPONSE {
                engine: "running".to_string(),
                ai: "connected".to_string(),
                grammar: "ready".to_string(),
            },
            IpcRequest::VARIABLE_LIST => {
                match self.variable_engine.list_all().await {
                    Ok(items) => {
                        let variables = items
                            .into_iter()
                            .map(|v| VariableDto {
                                key: v.key,
                                var_type: v.var_type.as_str().to_string(),
                                value: v.value,
                                ai_prompt: v.ai_prompt,
                                description: None,
                                use_count: v.use_count,
                            })
                            .collect();
                        IpcResponse::VARIABLE_LIST_RESPONSE { variables }
                    }
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::VARIABLE_UPSERT { variable } => {
                let var = keymind_variables::db::Variable {
                    key: variable.key.clone(),
                    var_type: match variable.var_type.as_str() {
                        "static" => keymind_variables::db::VarType::Static,
                        "dynamic" => keymind_variables::db::VarType::Dynamic,
                        "ai" => keymind_variables::db::VarType::Ai,
                        _ => keymind_variables::db::VarType::Static,
                    },
                    value: variable.value.clone(),
                    ai_prompt: variable.ai_prompt.clone(),
                    use_count: variable.use_count,
                    created_at: 0,
                    updated_at: 0,
                };

                match self.variable_engine.upsert(var).await {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::VARIABLE_DELETE { key } => {
                match self.variable_engine.delete(&key).await {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::STATS_REQUEST => {
                // Mock daily stats for now since we removed SQLite
                let today = DailyStatsDto {
                    words_typed: 0,
                    corrections_made: 0,
                    variables_used: 0,
                    ai_requests: 0,
                };
                IpcResponse::STATS_RESPONSE { today }
            }
            IpcRequest::LEARNED_PHRASES => {
                match self.learning_engine.db.get_learned_phrases().await {
                    Ok(items) => {
                        let phrases = items
                            .into_iter()
                            .map(|p| LearnedPhraseDto {
                                id: p.id,
                                phrase: p.phrase,
                                frequency: p.frequency as i64,
                                pinned: p.is_pinned,
                                category: None,
                            })
                            .collect();
                        IpcResponse::LEARNED_PHRASES_RESPONSE { phrases }
                    }
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::PIN_PHRASE { id } => {
                match self.learning_engine.db.pin_phrase(&id).await {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::DELETE_PHRASE { id } => {
                match self.learning_engine.db.delete_phrase(&id).await {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::TOGGLE_LEARNING { enabled } => {
                self.learning_enabled.store(enabled, Ordering::Relaxed);
                self.learning_engine.toggle_learning(enabled);
                IpcResponse::OK
            }
        }
    }

    /// Start listening on Unix Domain Socket.
    #[cfg(unix)]
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let p = Path::new(&self.socket_path);
        if p.exists() {
            let _ = fs::remove_file(p);
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        info!("IPC Server listening on UDS: {}", self.socket_path);

        let server_arc = Arc::new(self.clone_handle());

        loop {
            let (stream, _) = listener.accept().await?;
            let server = server_arc.clone();

            tokio::spawn(async move {
                let (reader, mut writer) = stream.into_split();
                let mut lines = BufReader::new(reader).lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(req) = serde_json::from_str::<IpcRequest>(&line) {
                        let resp = server.handle_request(req).await;
                        if let Ok(mut json_out) = serde_json::to_string(&resp) {
                            json_out.push('\n');
                            let _ = writer.write_all(json_out.as_bytes()).await;
                        }
                    }
                }
            });
        }
    }

    #[cfg(windows)]
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = tokio::net::TcpListener::bind(&self.socket_path).await?;
        info!("IPC Server listening on TCP: {}", self.socket_path);

        let server_arc = Arc::new(self.clone_handle());

        loop {
            let (stream, _) = listener.accept().await?;
            let server = server_arc.clone();

            tokio::spawn(async move {
                let (reader, mut writer) = stream.into_split();
                let mut lines = BufReader::new(reader).lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(req) = serde_json::from_str::<IpcRequest>(&line) {
                        let resp = server.handle_request(req).await;
                        if let Ok(mut json_out) = serde_json::to_string(&resp) {
                            json_out.push('\n');
                            let _ = writer.write_all(json_out.as_bytes()).await;
                        }
                    }
                }
            });
        }
    }

    fn clone_handle(&self) -> Self {
        Self {
            variable_engine: Arc::clone(&self.variable_engine),
            learning_engine: Arc::clone(&self.learning_engine),
            socket_path: self.socket_path.clone(),
            learning_enabled: Arc::clone(&self.learning_enabled),
        }
    }
}

pub async fn start_ipc_server(
    variable_engine: Arc<VariableEngine>,
    learning_engine: Arc<LearningEngine>,
    socket_path: &str,
    learning_enabled: Arc<AtomicBool>,
) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>> {
    let server = IpcServer::new(variable_engine, learning_engine, Some(socket_path), learning_enabled);
    let handle = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            tracing::error!("IPC Server failed: {}", e);
        }
    });
    Ok(handle)
}
