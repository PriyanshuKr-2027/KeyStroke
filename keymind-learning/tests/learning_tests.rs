use keymind_learning::{
    get_learned_phrases, init_learning_tables, LearningEngine, PrivacyFilter, TypingEvent,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_phrase_promotion_and_privacy_stream() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory sqlite");

    init_learning_tables(&pool).await.unwrap();
    let db = Arc::new(pool);

    let (tx, rx) = mpsc::channel(1000);
    let _handle = LearningEngine::start(db.clone(), rx);

    // 1. Send repeated phrase events (10 times to trigger promotion threshold)
    for _ in 0..10 {
        tx.send(TypingEvent {
            text: "quarterly financial results".to_string(),
            app_id: Some("com.apple.Notes".to_string()),
            is_sensitive: false,
        })
        .await
        .unwrap();
    }

    // 2. Send sensitive events (credit card and OTP)
    tx.send(TypingEvent {
        text: "My credit card is 4111-2222-3333-4444".to_string(),
        app_id: Some("com.apple.Safari".to_string()),
        is_sensitive: false,
    })
    .await
    .unwrap();

    tx.send(TypingEvent {
        text: "OTP code 938201".to_string(),
        app_id: Some("com.apple.Terminal".to_string()),
        is_sensitive: false,
    })
    .await
    .unwrap();

    tx.send(TypingEvent {
        text: "password123".to_string(),
        app_id: Some("com.apple.Keychain".to_string()),
        is_sensitive: true, // Should drop
    })
    .await
    .unwrap();

    // Give background worker thread time to process
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Verify promoted phrases
    let phrases = get_learned_phrases(&db).await.unwrap();
    assert!(!phrases.is_empty(), "Should contain promoted phrases");

    let contains_target = phrases
        .iter()
        .any(|p| p.phrase.to_lowercase().contains("financial results") || p.phrase.to_lowercase().contains("quarterly financial"));
    assert!(contains_target, "Target phrase should be promoted");

    // Verify privacy blocking
    let contains_credit_card = phrases
        .iter()
        .any(|p| p.phrase.contains("4111") || p.phrase.contains("4444"));
    assert!(!contains_credit_card, "Credit card should be blocked by privacy filter");

    let contains_otp = phrases.iter().any(|p| p.phrase.contains("938201"));
    assert!(!contains_otp, "OTP should be blocked by privacy filter");
}
