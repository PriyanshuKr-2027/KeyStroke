use keymind_ipc::{
    init_db_pool, DailyStatsDto, IpcRequest, IpcResponse, IpcServer, VariableDto,
};
use std::sync::Arc;

#[tokio::test]
async fn test_all_9_ipc_message_types() {
    let pool = init_db_pool(Some("sqlite::memory:")).await.unwrap();
    let server = IpcServer::new(pool, None);

    // 1. STATUS_REQUEST
    let resp = server.handle_request(IpcRequest::STATUS_REQUEST).await;
    assert_eq!(
        resp,
        IpcResponse::STATUS_RESPONSE {
            engine: "running".to_string(),
            ai: "connected".to_string(),
            grammar: "ready".to_string(),
        }
    );

    // 2. VARIABLE_UPSERT
    let var = VariableDto {
        key: "phone".to_string(),
        var_type: "static".to_string(),
        value: Some("+1-555-0199".to_string()),
        ai_prompt: None,
        description: Some("Phone number snippet".to_string()),
        use_count: 0,
    };
    let resp_upsert = server
        .handle_request(IpcRequest::VARIABLE_UPSERT { variable: var.clone() })
        .await;
    assert_eq!(resp_upsert, IpcResponse::OK);

    // 3. VARIABLE_LIST
    let resp_list = server.handle_request(IpcRequest::VARIABLE_LIST).await;
    match resp_list {
        IpcResponse::VARIABLE_LIST_RESPONSE { variables } => {
            assert_eq!(variables.len(), 1);
            assert_eq!(variables[0].key, "phone");
            assert_eq!(variables[0].value, Some("+1-555-0199".to_string()));
        }
        _ => panic!("Expected VARIABLE_LIST_RESPONSE"),
    }

    // 4. VARIABLE_DELETE
    let resp_del = server
        .handle_request(IpcRequest::VARIABLE_DELETE {
            key: "phone".to_string(),
        })
        .await;
    assert_eq!(resp_del, IpcResponse::OK);

    let resp_list_after = server.handle_request(IpcRequest::VARIABLE_LIST).await;
    match resp_list_after {
        IpcResponse::VARIABLE_LIST_RESPONSE { variables } => {
            assert_eq!(variables.len(), 0);
        }
        _ => panic!("Expected VARIABLE_LIST_RESPONSE"),
    }

    // 5. STATS_REQUEST
    let resp_stats = server.handle_request(IpcRequest::STATS_REQUEST).await;
    match resp_stats {
        IpcResponse::STATS_RESPONSE { today } => {
            assert_eq!(today.words_typed, 0);
        }
        _ => panic!("Expected STATS_RESPONSE"),
    }

    // 6. LEARNED_PHRASES
    let resp_phrases = server.handle_request(IpcRequest::LEARNED_PHRASES).await;
    match resp_phrases {
        IpcResponse::LEARNED_PHRASES_RESPONSE { phrases } => {
            assert_eq!(phrases.len(), 0);
        }
        _ => panic!("Expected LEARNED_PHRASES_RESPONSE"),
    }

    // 7. PIN_PHRASE
    let resp_pin = server
        .handle_request(IpcRequest::PIN_PHRASE {
            id: "phrase_123".to_string(),
        })
        .await;
    assert_eq!(resp_pin, IpcResponse::OK);

    // 8. DELETE_PHRASE
    let resp_del_phrase = server
        .handle_request(IpcRequest::DELETE_PHRASE {
            id: "phrase_123".to_string(),
        })
        .await;
    assert_eq!(resp_del_phrase, IpcResponse::OK);

    // 9. TOGGLE_LEARNING
    let resp_toggle = server
        .handle_request(IpcRequest::TOGGLE_LEARNING { enabled: true })
        .await;
    assert_eq!(resp_toggle, IpcResponse::OK);
}

#[tokio::test]
async fn test_json_serialization_roundtrip() {
    let req = IpcRequest::STATUS_REQUEST;
    let json_req = serde_json::to_string(&req).unwrap();
    assert_eq!(json_req, r#"{"type":"STATUS_REQUEST"}"#);

    let parsed_req: IpcRequest = serde_json::from_str(&json_req).unwrap();
    match parsed_req {
        IpcRequest::STATUS_REQUEST => (),
        _ => panic!("Roundtrip failed"),
    }

    let resp = IpcResponse::OK;
    let json_resp = serde_json::to_string(&resp).unwrap();
    assert_eq!(json_resp, r#"{"type":"OK"}"#);
}
