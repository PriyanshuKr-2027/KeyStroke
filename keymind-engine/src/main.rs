pub mod pipeline;

use keymind_autocorrect::AutocorrectEngine;
use keymind_grammar::GrammarEngine;
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
    let _ = dotenvy::dotenv();

    // 2. Instantiate Sub-Crates
    let autocorrect_path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("keymind")
        .join("autocorrect.json");
    let autocorrect = Arc::new(AutocorrectEngine::new(autocorrect_path));
    if let Err(e) = autocorrect.initialize().await {
        tracing::error!("Failed to initialize Autocorrect Engine: {}", e);
    }
    
    let variables_path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("keymind")
        .join("variables.json");
    let variables = Arc::new(VariableEngine::new(variables_path));
    if let Err(e) = variables.initialize().await {
        tracing::error!("Failed to initialize Variable Engine: {}", e);
    }
    let grammar = Arc::new(GrammarEngine::with_java_server(PathBuf::from(
        "app_resources/languagetool/languagetool-server.jar",
    ), 8081));

    let prediction_path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("keymind")
        .join("trigrams.json");
    let prediction = Arc::new(
        PredictionEngine::new(
            prediction_path,
            PathBuf::from("app_resources/models/gpt2-int8.onnx"),
        )
        .await?,
    );

    // 3. Start Learning Engine Worker
    let learning_path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("keymind")
        .join("learning.json");
    let (learning_tx, learning_rx) = mpsc::channel(1000);
    let learning_engine = Arc::new(LearningEngine::new(learning_path));
    if let Err(e) = learning_engine.initialize().await {
        tracing::error!("Failed to initialize Learning Engine: {}", e);
    }
    let _learning_task = learning_engine.start(learning_rx);

    // 4. Initialize Typing Pipeline Controller
    let pipeline = Arc::new(TypingPipeline::new(
        autocorrect,
        variables.clone(),
        grammar,
        prediction,
        learning_tx,
    ));

    // 5. Start IPC Server Daemon
    #[cfg(windows)]
    let ipc_address = "127.0.0.1:9123";
    #[cfg(not(windows))]
    let ipc_address = "/tmp/keymind.sock";

    let _ipc_task = start_ipc_server(variables.clone(), learning_engine.clone(), ipc_address, learning_engine.enabled.clone()).await?;
    info!("KeyMind IPC Server daemon listening on {}", ipc_address);

    // 7. Start Windows / macOS Keyboard Interceptor
    #[cfg(target_os = "windows")]
    {
        use keymind_interceptor_windows::KeymindWindowsInterceptor;
        let (mut rx, _handle, injector) = KeymindWindowsInterceptor::start(100);
        info!("Windows Keyboard Interceptor (WH_KEYBOARD_LL) active.");
        let pipeline_ref = pipeline.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    keymind_interceptor_windows::Event::WordCompleted { word, context } => {
                        pipeline_ref.process_word(&word, &context, false).await;
                    }
                    keymind_interceptor_windows::Event::SensitiveFieldKeyPress => {
                        // Skip processing for password fields
                    }
                    _ => {}
                }
            }
        });
        let _ = injector;
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
