pub mod context;
pub mod focus;
pub mod inject;
pub mod window;

pub use context::{capture_context, CapturedContext};
pub use focus::{get_focused_window, restore_focus, ActiveWindowHandle};
pub use inject::inject_text;
pub use window::{close_palette, open_palette};
