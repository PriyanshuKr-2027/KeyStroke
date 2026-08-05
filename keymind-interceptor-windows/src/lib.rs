pub mod context;
pub mod events;
pub mod hook;
pub mod injector;
pub mod lifecycle;
pub mod service;

pub use context::ContextBuffer;
pub use events::{Event, Modifiers};
pub use hook::{is_focused_element_secure, translate_vk_code};
pub use injector::TextInjector;
pub use lifecycle::{start_interceptor, HookHandle};
pub use service::{service_main, SERVICE_NAME};

use tokio::sync::mpsc;

pub struct KeymindWindowsInterceptor;

impl KeymindWindowsInterceptor {
    pub fn start(channel_capacity: usize) -> (mpsc::Receiver<Event>, HookHandle, TextInjector) {
        let (tx, rx) = mpsc::channel(channel_capacity);
        let handle = start_interceptor(tx);
        (rx, handle, TextInjector::new())
    }
}
