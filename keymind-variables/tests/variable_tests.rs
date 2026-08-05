use keymind_variables::ai::MockGroqClient;
use keymind_variables::db::{VarType, Variable};
use keymind_variables::dynamic::DynamicResolver;
use keymind_variables::{ExpansionTask, VariableEngine};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

async fn setup_test_engine() -> (VariableEngine, Arc<MockGroqClient>) {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mock_client = Arc::new(MockGroqClient::new());
    let engine = VariableEngine::with_ai_client(Arc::new(pool), mock_client.clone());
    engine.initialize().await.unwrap();
    (engine, mock_client)
}

#[tokio::test]
async fn test_trigger_detection_static_and_dynamic() {
    let (engine, _) = setup_test_engine().await;

    // Register a static variable
    let static_var = Variable {
        key: "sig".to_string(),
        var_type: VarType::Static,
        value: Some("Best regards,\nJohn Doe".to_string()),
        ai_prompt: None,
        use_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    engine.upsert(static_var).await.unwrap();

    // Test static trigger typing "/sig "
    for c in "/sig".chars() {
        assert!(engine.process_keystroke(c).is_none());
    }
    let task = engine.process_keystroke(' ').expect("Should trigger static expansion");
    assert_eq!(
        task,
        ExpansionTask::Static {
            backspace_count: 5, // '/' + "sig" + 1
            replacement: "Best regards,\nJohn Doe".to_string(),
        }
    );

    // Test dynamic trigger typing "/date "
    for c in "/date".chars() {
        assert!(engine.process_keystroke(c).is_none());
    }
    let task_dyn = engine.process_keystroke(' ').expect("Should trigger dynamic expansion");
    match task_dyn {
        ExpansionTask::Dynamic { backspace_count, replacement } => {
            assert_eq!(backspace_count, 6);
            assert!(!replacement.is_empty());
        }
        _ => panic!("Expected dynamic task"),
    }
}

#[tokio::test]
async fn test_all_5_ai_variable_types() {
    let (engine, _) = setup_test_engine().await;

    let ai_variables = vec!["leave", "reply", "meeting", "summarize", "translate"];
    let sample_clipboard = "Subject: Project update request";

    for key in ai_variables {
        // Test trigger detection
        for c in format!("/{}", key).chars() {
            assert!(engine.process_keystroke(c).is_none());
        }
        let task = engine.process_keystroke(' ').expect("Should trigger AI expansion");

        match task {
            ExpansionTask::Ai { key: k, backspace_count, .. } => {
                assert_eq!(k, key);
                assert_eq!(backspace_count, 1 + key.len() + 1);
            }
            _ => panic!("Expected AI task for {}", key),
        }

        // Test AI resolution via MockGroqClient
        let res = engine.resolve_ai(key, sample_clipboard).await;
        assert!(res.is_ok(), "AI variable {} should resolve successfully", key);
        let output = res.unwrap();
        assert!(!output.is_empty(), "AI output for {} should not be empty", key);
    }
}

#[tokio::test]
async fn test_dynamic_variable_resolvers() {
    assert!(DynamicResolver::resolve("date").is_some());
    assert!(DynamicResolver::resolve("time").is_some());
    assert!(DynamicResolver::resolve("day").is_some());
    assert!(DynamicResolver::resolve("uuid").is_some());
    assert!(DynamicResolver::resolve("timestamp").is_some());
    assert!(DynamicResolver::resolve("clipboard").is_some());
}

#[tokio::test]
async fn test_crud_operations() {
    let (engine, _) = setup_test_engine().await;

    let var = Variable {
        key: "testkey".to_string(),
        var_type: VarType::Static,
        value: Some("test value".to_string()),
        ai_prompt: None,
        use_count: 0,
        created_at: 0,
        updated_at: 0,
    };

    engine.upsert(var).await.unwrap();
    assert_eq!(engine.resolve_static("testkey"), Some("test value".to_string()));

    let list = engine.list_all().await.unwrap();
    assert_eq!(list.len(), 1);

    engine.delete("testkey").await.unwrap();
    assert_eq!(engine.resolve_static("testkey"), None);
}
