use parking_lot::Mutex;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

pub struct ServerProcessManager {
    jar_path: PathBuf,
    port: u16,
    child_handle: Arc<Mutex<Option<Child>>>,
    client: reqwest::Client,
}

impl ServerProcessManager {
    pub fn new(jar_path: PathBuf, port: u16) -> Self {
        Self {
            jar_path,
            port,
            child_handle: Arc::new(Mutex::new(None)),
            client: reqwest::Client::new(),
        }
    }

    /// Spawns the Java LanguageTool process and waits for health check 200 OK up to 15 seconds.
    pub async fn start_server(&self) -> bool {
        if !self.jar_path.exists() {
            warn!("LanguageTool JAR path does not exist: {:?}", self.jar_path);
            return false;
        }

        let mut child_lock = self.child_handle.lock();
        if child_lock.is_some() {
            info!("LanguageTool server process already running.");
            return true;
        }

        let child = Command::new("java")
            .arg("-cp")
            .arg(&self.jar_path)
            .arg("org.languagetool.server.HTTPServer")
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--allow-origin")
            .arg("*")
            .spawn();

        match child {
            Ok(c) => {
                *child_lock = Some(c);
                drop(child_lock);

                // Health check poll up to 15s (500ms interval)
                self.poll_health_check(15, 500).await
            }
            Err(e) => {
                warn!("Failed to spawn LanguageTool java process: {}", e);
                false
            }
        }
    }

    /// Polls GET http://localhost:{port}/v2/languages until 200 OK or max attempts reached.
    pub async fn poll_health_check(&self, max_attempts: usize, interval_ms: u64) -> bool {
        let url = format!("http://localhost:{}/v2/languages", self.port);

        for attempt in 1..=max_attempts {
            if let Ok(res) = self.client.get(&url).send().await {
                if res.status().is_success() {
                    info!("LanguageTool server health check passed (attempt {}).", attempt);
                    return true;
                }
            }
            sleep(Duration::from_millis(interval_ms)).await;
        }

        warn!("LanguageTool server health check timed out after 15s.");
        false
    }

    /// Stop/kill process gracefully.
    pub fn stop_server(&self) {
        let mut child_lock = self.child_handle.lock();
        if let Some(mut child) = child_lock.take() {
            let _ = child.kill();
            let _ = child.wait();
            info!("Killed LanguageTool server process.");
        }
    }

    /// Restart process with backoff logic (2s, 4s, 8s).
    pub async fn restart_with_backoff(&self, attempt: usize) -> bool {
        let secs = match attempt {
            0 => 2,
            1 => 4,
            _ => 8,
        };

        info!("Restarting LanguageTool server (backoff {}s)...", secs);
        sleep(Duration::from_secs(secs)).await;
        self.stop_server();
        self.start_server().await
    }
}

impl Drop for ServerProcessManager {
    fn drop(&mut self) {
        self.stop_server();
    }
}
