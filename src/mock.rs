use crate::context::ContextBuffer;
use crate::events::{Event, Modifiers};
use std::time::Duration;
use tokio::sync::mpsc;

/// Simulated event source for testing context tracking, security checks, and backoff handling without macOS GUI.
pub struct MockEventSource {
    sender: mpsc::Sender<Event>,
    context_buffer: ContextBuffer,
    is_secure_mode: bool,
    consecutive_failures: usize,
}

impl MockEventSource {
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self {
            sender,
            context_buffer: ContextBuffer::default(),
            is_secure_mode: false,
            consecutive_failures: 0,
        }
    }

    /// Set secure field simulation mode.
    pub fn set_secure_mode(&mut self, secure: bool) {
        self.is_secure_mode = secure;
    }

    /// Simulate a keypress event.
    pub async fn simulate_keypress(&mut self, c: char, modifiers: Modifiers) -> Result<(), mpsc::error::SendError<Event>> {
        if self.is_secure_mode {
            self.sender.send(Event::SensitiveFieldKeyPress).await?;
        } else {
            self.sender.send(Event::KeyPress { key: c, modifiers }).await?;

            if let Some((word, context)) = self.context_buffer.push_char(c) {
                self.sender.send(Event::WordCompleted { word, context }).await?;
            }
        }
        Ok(())
    }

    /// Simulate typing a full string.
    pub async fn simulate_type_string(&mut self, text: &str) -> Result<(), mpsc::error::SendError<Event>> {
        for c in text.chars() {
            self.simulate_keypress(c, Modifiers::default()).await?;
        }
        Ok(())
    }

    /// Simulate tap invalidation and trigger backoff reconnection logic.
    pub async fn simulate_tap_failure(&mut self) -> Option<Duration> {
        if self.consecutive_failures < 5 {
            let attempt = self.consecutive_failures;
            self.consecutive_failures += 1;
            Some(crate::tap::get_backoff_duration(attempt))
        } else {
            let _ = self.sender.send(Event::EngineError("tap_dead")).await;
            None
        }
    }

    /// Reset failure count.
    pub fn reset_failures(&mut self) {
        self.consecutive_failures = 0;
    }
}
