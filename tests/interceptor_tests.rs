use keymind_interceptor_macos::{
    Event, MockEventSource, Modifiers, TextInjector,
};
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_mock_typing_and_word_completion() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut mock = MockEventSource::new(tx);

    mock.simulate_type_string("hello world ").await.unwrap();

    let mut events = Vec::new();
    while let Ok(evt) = rx.try_recv() {
        events.push(evt);
    }

    // Ensure we receive KeyPress events and WordCompleted events
    let word_events: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            Event::WordCompleted { word, context } => Some((word.as_str(), context.as_str())),
            _ => None,
        })
        .collect();

    assert_eq!(word_events.len(), 2);
    assert_eq!(word_events[0], ("hello", "hello "));
    assert_eq!(word_events[1], ("world", "hello world "));
}

#[tokio::test]
async fn test_sensitive_field_key_press() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut mock = MockEventSource::new(tx);

    mock.set_secure_mode(true);
    mock.simulate_keypress('p', Modifiers::default()).await.unwrap();

    let evt = rx.recv().await.expect("Should receive sensitive field event");
    assert_eq!(evt, Event::SensitiveFieldKeyPress);
}

#[tokio::test]
async fn test_exponential_backoff_and_dead_tap() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut mock = MockEventSource::new(tx);

    let expected_delays = vec![
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(4),
        Duration::from_secs(8),
        Duration::from_secs(16),
    ];

    for expected in expected_delays {
        let delay = mock.simulate_tap_failure().await.expect("Should return backoff duration");
        assert_eq!(delay, expected);
    }

    // 6th failure should emit EngineError("tap_dead")
    let dead_result = mock.simulate_tap_failure().await;
    assert!(dead_result.is_none());

    let evt = rx.recv().await.expect("Should receive dead tap event");
    assert_eq!(evt, Event::EngineError("tap_dead"));
}

#[test]
fn test_text_injector_api() {
    let injector = TextInjector::new();
    injector.inject_text("test injection");
    injector.send_backspaces(5);
}
