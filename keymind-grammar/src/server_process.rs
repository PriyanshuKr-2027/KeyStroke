use std::path::PathBuf;

/// Legacy ServerProcessManager stub preserved for API compatibility.
/// Java process is no longer needed as KeyStroke uses native nlprule in Rust.
pub struct ServerProcessManager;

impl ServerProcessManager {
    pub fn new(_jar_path: PathBuf, _port: u16) -> Self {
        Self
    }

    pub async fn start_server(&self) -> bool {
        true
    }

    pub fn stop_server(&self) {}
}
