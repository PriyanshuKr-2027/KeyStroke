use crate::db::SqlitePool;
use crate::protocol::{
    DailyStatsDto, IpcRequest, IpcResponse, LearnedPhraseDto, VariableDto,
};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
#[cfg(unix)]
use tokio::net::UnixListener;
use tracing::info;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct IpcServer {
    pool: Arc<SqlitePool>,
    socket_path: String,
    learning_enabled: Arc<AtomicBool>,
}

impl IpcServer {
    pub fn new(pool: Arc<SqlitePool>, socket_path: Option<&str>, learning_enabled: Arc<AtomicBool>) -> Self {
        let path = socket_path.unwrap_or(if cfg!(windows) { "127.0.0.1:9123" } else { "/tmp/keymind.sock" }).to_string();
        Self {
            pool,
            socket_path: path,
            learning_enabled,
        }
    }

    /// Process a single incoming IpcRequest against SQLite pool.
    pub async fn handle_request(&self, request: IpcRequest) -> IpcResponse {
        match request {
            IpcRequest::STATUS_REQUEST => IpcResponse::STATUS_RESPONSE {
                engine: "running".to_string(),
                ai: "connected".to_string(),
                grammar: "ready".to_string(),
            },
            IpcRequest::VARIABLE_LIST => {
                let rows: Result<Vec<(String, String, Option<String>, Option<String>, Option<String>, i64)>, _> =
                    sqlx::query_as(
                        "SELECT key, var_type, value, ai_prompt, description, use_count FROM variables",
                    )
                    .fetch_all(self.pool.as_ref())
                    .await;

                match rows {
                    Ok(items) => {
                        let variables = items
                            .into_iter()
                            .map(|(key, var_type, value, ai_prompt, description, use_count)| {
                                VariableDto {
                                    key,
                                    var_type,
                                    value,
                                    ai_prompt,
                                    description,
                                    use_count,
                                }
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
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;

                let res = sqlx::query(
                    "INSERT INTO variables (key, var_type, value, ai_prompt, description, use_count, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                     ON CONFLICT(key) DO UPDATE SET
                     var_type = excluded.var_type,
                     value = excluded.value,
                     ai_prompt = excluded.ai_prompt,
                     description = excluded.description,
                     updated_at = excluded.updated_at",
                )
                .bind(&variable.key)
                .bind(&variable.var_type)
                .bind(&variable.value)
                .bind(&variable.ai_prompt)
                .bind(&variable.description)
                .bind(variable.use_count)
                .bind(now)
                .bind(now)
                .execute(self.pool.as_ref())
                .await;

                match res {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::VARIABLE_DELETE { key } => {
                let res = sqlx::query("DELETE FROM variables WHERE key = ?")
                    .bind(&key)
                    .execute(self.pool.as_ref())
                    .await;

                match res {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::STATS_REQUEST => {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                let row: Option<(i64, i64, i64, i64)> = sqlx::query_as(
                    "SELECT words_typed, corrections_made, variables_used, ai_requests FROM daily_stats WHERE date = ?",
                )
                .bind(&today_str)
                .fetch_optional(self.pool.as_ref())
                .await
                .unwrap_or(None);

                let today = match row {
                    Some((words_typed, corrections_made, variables_used, ai_requests)) => {
                        DailyStatsDto {
                            words_typed,
                            corrections_made,
                            variables_used,
                            ai_requests,
                        }
                    }
                    None => DailyStatsDto::default(),
                };

                IpcResponse::STATS_RESPONSE { today }
            }
            IpcRequest::LEARNED_PHRASES => {
                let rows: Result<Vec<(String, String, i64, bool, Option<String>)>, _> =
                    sqlx::query_as("SELECT id, phrase, frequency, pinned, category FROM learned_memory")
                        .fetch_all(self.pool.as_ref())
                        .await;

                match rows {
                    Ok(items) => {
                        let phrases = items
                            .into_iter()
                            .map(|(id, phrase, frequency, pinned, category)| LearnedPhraseDto {
                                id,
                                phrase,
                                frequency,
                                pinned,
                                category,
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
                let res = sqlx::query("UPDATE learned_memory SET pinned = 1 WHERE id = ?")
                    .bind(&id)
                    .execute(self.pool.as_ref())
                    .await;

                match res {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::DELETE_PHRASE { id } => {
                let res = sqlx::query("DELETE FROM learned_memory WHERE id = ?")
                    .bind(&id)
                    .execute(self.pool.as_ref())
                    .await;

                match res {
                    Ok(_) => IpcResponse::OK,
                    Err(e) => IpcResponse::ERROR {
                        message: e.to_string(),
                    },
                }
            }
            IpcRequest::TOGGLE_LEARNING { enabled } => {
                self.learning_enabled.store(enabled, Ordering::Relaxed);
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
            pool: Arc::clone(&self.pool),
            socket_path: self.socket_path.clone(),
            learning_enabled: Arc::clone(&self.learning_enabled),
        }
    }
}

pub async fn start_ipc_server(
    pool: Arc<SqlitePool>,
    socket_path: &str,
    learning_enabled: Arc<AtomicBool>,
) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error + Send + Sync>> {
    let server = IpcServer::new(pool, Some(socket_path), learning_enabled);
    let handle = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            tracing::error!("IPC Server failed: {}", e);
        }
    });
    Ok(handle)
}
