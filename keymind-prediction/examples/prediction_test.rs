use keymind_prediction::PredictionEngine;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("==================================================================");
    println!(" KeyMind Next-Word Prediction Engine Test");
    println!("==================================================================\n");

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    let engine = PredictionEngine::new(Arc::new(pool), PathBuf::from("model.onnx"))
        .await
        .expect("Failed to initialize Prediction engine");

    let test_contexts = vec![
        "how are",
        "thank you",
        "please let",
        "in the",
    ];

    println!("{:<20} | {:<30} | {:<10}", "Input Context", "Predicted Next Words", "Confidence");
    println!("----------------------------------------------------------------------------------");

    for ctx in test_contexts {
        let (suggestions, conf) = engine.predict(ctx).await;
        let suggestion_str = if suggestions.is_empty() {
            "[No prediction]".to_string()
        } else {
            suggestions.join(", ")
        };
        println!("{:<20} | {:<30} | {:.0}%", ctx, suggestion_str, conf * 100.0);
    }

    println!("\n==================================================================");
}
