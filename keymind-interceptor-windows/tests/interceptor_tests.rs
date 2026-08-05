use keymind_interceptor_windows::{
    ContextBuffer, Event, KeymindWindowsInterceptor, TextInjector,
};
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_context_buffer_word_completion() {
    let mut ctx = ContextBuffer::default();

    let mut completed = None;
    for c in "hello world ".chars() {
        if let Some(res) = ctx.push_char(c) {
            completed = Some(res);
        }
    }

    let (word, context) = completed.expect("Should produce word completion");
    assert_eq!(word, "world");
    assert_eq!(context, "hello world ");
}

#[test]
fn test_text_injector_api() {
    let injector = TextInjector::new();
    injector.inject_text("Windows test injection");
    injector.send_backspaces(4);
}

#[tokio::test]
async fn test_interceptor_start_stop_lifecycle() {
    let (_rx, handle, injector) = KeymindWindowsInterceptor::start(100);
    assert!(handle.is_running());

    injector.inject_text("a");
    tokio::time::sleep(Duration::from_millis(50)).await;

    handle.stop();
}
