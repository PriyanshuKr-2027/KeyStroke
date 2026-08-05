pub mod pipeline;

use keymind_autocorrect::AutocorrectEngine;
use keymind_grammar::GrammarEngine;
use keymind_ipc::db::init_db;
use keymind_ipc::server::start_ipc_server;
use keymind_learning::LearningEngine;
use keymind_prediction::PredictionEngine;
use keymind_variables::VariableEngine;
use pipeline::TypingPipeline;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::mpsc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting KeyMind Engine Orchestrator...");

    // Load .env if present
    if let Some(home) = dirs_next::home_dir() {
        let env_path = home.join(".config").join("keymind").join(".env");
        let _ = dotenvy::from_path(env_path);
    }

    // 2. Initialize SQLite Database
    let db_pool = Arc::new(init_db().await?);
    info!("SQLite database initialized and migrations applied.");

    // 3. Instantiate Sub-Crates
    let autocorrect = Arc::new(AutocorrectEngine::new(db_pool.clone()));
    let variables = Arc::new(VariableEngine::new(db_pool.clone()));
    let grammar = Arc::new(GrammarEngine::new(PathBuf::from(
        "app_resources/languagetool/languagetool-server.jar",
    )));

    let prediction = Arc::new(
        PredictionEngine::new(
            db_pool.clone(),
            PathBuf::from("app_resources/models/gpt2-int8.onnx"),
        )
        .await?,
    );

    // 4. Start Learning Engine Worker
    let (learning_tx, learning_rx) = mpsc::channel(1000);
    let _learning_task = LearningEngine::start(db_pool.clone(), learning_rx);

    // 5. Initialize Typing Pipeline Controller
    let _pipeline = Arc::new(TypingPipeline::new(
        autocorrect,
        variables,
        grammar,
        prediction,
        learning_tx,
    ));

    // 6. Start IPC Server Daemon (/tmp/keymind.sock)
    let _ipc_task = start_ipc_server(db_pool.clone(), "/tmp/keymind.sock").await?;
    info!("KeyMind IPC Server daemon listening on /tmp/keymind.sock");

    // 7. Start Windows / macOS Keyboard Interceptor
    #[cfg(target_os = "windows")]
    {
        use keymind_interceptor_windows::KeymindWindowsInterceptor;
        let (_rx, _handle, _injector) = KeymindWindowsInterceptor::start(100);
        info!("Windows Keyboard Interceptor (WH_KEYBOARD_LL) active.");
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("macOS CGEventTap Interceptor active.");
    }

    info!("KeyMind Engine is running. Press Ctrl+C to shutdown.");

    // 8. Wait for Shutdown Signal
    signal::ctrl_c().await?;
    info!("KeyMind Engine shutting down gracefully.");

    Ok(())
}
