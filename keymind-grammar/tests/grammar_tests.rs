use keymind_grammar::fixer::apply_text_fixes;
use keymind_grammar::{GrammarEngine, GrammarIssue};

#[tokio::test]
async fn test_grammar_engine_cache_and_fixer() {
    let engine = GrammarEngine::new(8081);

    let text = "He are going to teh store.";
    let issues = vec![
        GrammarIssue {
            offset: 3,
            length: 3,
            message: "Did you mean 'is'?".to_string(),
            replacements: vec!["is".to_string()],
            rule_id: "HE_ARE".to_string(),
            category: "GRAMMAR".to_string(),
        },
        GrammarIssue {
            offset: 16,
            length: 3,
            message: "Did you mean 'the'?".to_string(),
            replacements: vec!["the".to_string()],
            rule_id: "TEH_TYPO".to_string(),
            category: "TYPOS".to_string(),
        },
    ];

    let fixed = apply_text_fixes(text, &issues);
    assert_eq!(fixed, "He is going to the store.");
}

#[tokio::test]
async fn test_mock_languagetool_server_integration() {
    use tokio::net::TcpListener;

    // Spin up mock LanguageTool server on available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let mut buf = [0u8; 1024];
            let _ = socket.read(&mut buf).await;

            let response_body = r#"{
                "matches": [
                    {
                        "offset": 3,
                        "length": 3,
                        "message": "Did you mean 'is'?",
                        "replacements": [{"value": "is"}],
                        "rule": {
                            "id": "HE_ARE",
                            "category": {"id": "GRAMMAR"}
                        }
                    }
                ]
            }"#;

            let http_response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            );

            let _ = socket.write_all(http_response.as_bytes()).await;
        }
    });

    let engine = GrammarEngine::new(port);
    let issues = engine.check_text("He are going", "en-US").await;

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].rule_id, "HE_ARE");
    assert_eq!(issues[0].replacements[0], "is");

    let fixed = engine.fix_text("He are going").await;
    assert_eq!(fixed, "He is going");
}
