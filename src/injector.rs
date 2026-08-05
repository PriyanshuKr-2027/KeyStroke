/// Text injection engine for sending simulated keystrokes and backspaces.
#[derive(Clone, Default)]
pub struct TextInjector;

impl TextInjector {
    pub fn new() -> Self {
        Self
    }

    /// Inject arbitrary text by simulating CGEvent keyboard events per character.
    pub fn inject_text(&self, text: &str) {
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok();
            for c in text.chars() {
                let mut utf16_buf = [0u16; 2];
                let utf16 = c.encode_utf16(&mut utf16_buf);

                if let Ok(event_down) = CGEvent::new_keyboard_event(source.clone(), 0, true) {
                    event_down.keyboard_set_unicode_string(utf16);
                    event_down.post(CGEventTapLocation::HID);
                }

                if let Ok(event_up) = CGEvent::new_keyboard_event(source.clone(), 0, false) {
                    event_up.keyboard_set_unicode_string(utf16);
                    event_up.post(CGEventTapLocation::HID);
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            tracing::info!("[Mock Injector] Injected text: \"{}\"", text);
        }
    }

    /// Send `n` backspace key presses (VK_DELETE keycode 51 on macOS).
    pub fn send_backspaces(&self, n: usize) {
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

            const VK_DELETE: u16 = 51;
            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok();

            for _ in 0..n {
                if let Ok(event_down) = CGEvent::new_keyboard_event(source.clone(), VK_DELETE, true) {
                    event_down.post(CGEventTapLocation::HID);
                }
                if let Ok(event_up) = CGEvent::new_keyboard_event(source.clone(), VK_DELETE, false) {
                    event_up.post(CGEventTapLocation::HID);
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            tracing::info!("[Mock Injector] Sent {} backspaces", n);
        }
    }
}
