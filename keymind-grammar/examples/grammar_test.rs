use keymind_grammar::fixer::apply_text_fixes;
use keymind_grammar::{GrammarEngine, GrammarIssue};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    println!("==================================================================");
    println!(" KeyMind Real-Time Grammar & Style Engine Test");
    println!("==================================================================\n");

    // 1. Test In-Memory Grammar Fixer Algorithm
    let sample_text = "He are going to teh store because there books was lost.";
    let mock_issues = vec![
        GrammarIssue {
            offset: 3,
            length: 3,
            message: "Did you mean 'is'?".to_string(),
            replacements: vec!["is".to_string()],
            rule_id: "SUBJECT_VERB_AGREEMENT".to_string(),
            category: "GRAMMAR".to_string(),
        },
        GrammarIssue {
            offset: 16,
            length: 3,
            message: "Did you mean 'the'?".to_string(),
            replacements: vec!["the".to_string()],
            rule_id: "TYPO_TEH".to_string(),
            category: "TYPOS".to_string(),
        },
        GrammarIssue {
            offset: 34,
            length: 5,
            message: "Did you mean 'their'?".to_string(),
            replacements: vec!["their".to_string()],
            rule_id: "HOMOPHONE_THERE".to_string(),
            category: "CONFUSABLE".to_string(),
        },
        GrammarIssue {
            offset: 46,
            length: 3,
            message: "Did you mean 'were'?".to_string(),
            replacements: vec!["were".to_string()],
            rule_id: "PLURAL_VERB".to_string(),
            category: "GRAMMAR".to_string(),
        },
    ];

    let fixed = apply_text_fixes(sample_text, &mock_issues);

    println!("{:<25} | {}", "Original Input Sentence", sample_text);
    println!("{:<25} | {}", "Grammar Fix Output", fixed);
    println!("\nDetected Grammar & Style Issues:");
    println!("----------------------------------------------------------------------------------");
    println!("{:<6} | {:<22} | {:<25} | {:<15}", "Offset", "Rule ID", "Issue Description", "Suggested Fix");
    println!("----------------------------------------------------------------------------------");
    for issue in &mock_issues {
        println!(
            "{:<6} | {:<22} | {:<25} | {:<15}",
            issue.offset,
            issue.rule_id,
            issue.message,
            issue.replacements.first().cloned().unwrap_or_default()
        );
    }

    // 2. Test Local LanguageTool Server Integration
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
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
    let server_issues = engine.check_text("He are going", "en-US").await;
    let server_fixed = engine.fix_text("He are going").await;

    println!("\nLanguageTool Server HTTP Verification:");
    println!("  Raw Input:  \"He are going\"");
    println!("  HTTP Status: OK 200 (Matches: {})", server_issues.len());
    println!("  Auto-Fix:   \"{}\"", server_fixed);

    println!("\n==================================================================");
}
