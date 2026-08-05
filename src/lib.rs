pub mod accessibility;
pub mod context;
pub mod events;
pub mod injector;
pub mod mock;
pub mod tap;

pub use accessibility::{is_accessibility_granted, is_focused_element_secure, open_accessibility_settings};
pub use context::ContextBuffer;
pub use events::{Event, Modifiers};
pub use injector::TextInjector;
pub use mock::MockEventSource;

use tokio::sync::mpsc;

/// Primary interceptor engine controller.
pub struct KeymindInterceptor {
    pub receiver: mpsc::Receiver<Event>,
    pub injector: TextInjector,
}

impl KeymindInterceptor {
    pub fn new(receiver: mpsc::Receiver<Event>, injector: TextInjector) -> Self {
        Self { receiver, injector }
    }

    /// Spawns the CGEventTap background thread and returns the channel receiver and text injector.
    pub fn start(channel_capacity: usize) -> (mpsc::Receiver<Event>, TextInjector) {
        let (tx, rx) = mpsc::channel(channel_capacity);
        tap::spawn_event_tap_thread(tx);
        (rx, TextInjector::new())
    }
}
