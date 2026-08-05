use arboard::Clipboard;
use tracing::info;

pub struct SelectionFixer;

impl SelectionFixer {
    /// Simulates Copy (Ctrl+C / Cmd+C), reads clipboard, applies fixes, and pastes (Ctrl+V / Cmd+V).
    pub async fn fix_selection<F, Fut>(fix_fn: F) -> Result<(), String>
    where
        F: FnOnce(String) -> Fut,
        Fut: std::future::Future<Output = String>,
    {
        // 1. Simulate Ctrl+C / Cmd+C
        Self::simulate_copy();

        // Small delay for clipboard synchronization
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // 2. Read clipboard
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        let original_text = clipboard.get_text().map_err(|e| e.to_string())?;

        if original_text.trim().is_empty() {
            return Ok(());
        }

        // 3. Fix text
        let corrected_text = fix_fn(original_text).await;

        // 4. Write back to clipboard
        clipboard.set_text(corrected_text).map_err(|e| e.to_string())?;

        // 5. Simulate Ctrl+V / Cmd+V
        Self::simulate_paste();

        info!("Selection fix completed successfully.");
        Ok(())
    }

    fn simulate_copy() {
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

            const VK_C: u16 = 8;
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event_down) = CGEvent::new_keyboard_event(Some(source.clone()), VK_C, true) {
                    event_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                    event_down.post(CGEventTapLocation::HID);
                }
                if let Ok(event_up) = CGEvent::new_keyboard_event(Some(source), VK_C, false) {
                    event_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                    event_up.post(CGEventTapLocation::HID);
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::thread;
            use std::time::Duration;
            thread::sleep(Duration::from_millis(50));
            unsafe {
                use windows::Win32::UI::Input::KeyboardAndMouse::*;
                let inputs = [
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x11), ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x43), ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x43), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x11), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
                ];
                SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            }
            thread::sleep(Duration::from_millis(50));
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            info!("[Mock Shortcut] Simulated Ctrl+C copy");
        }
    }

    fn simulate_paste() {
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

            const VK_V: u16 = 9;
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event_down) = CGEvent::new_keyboard_event(Some(source.clone()), VK_V, true) {
                    event_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                    event_down.post(CGEventTapLocation::HID);
                }
                if let Ok(event_up) = CGEvent::new_keyboard_event(Some(source), VK_V, false) {
                    event_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                    event_up.post(CGEventTapLocation::HID);
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::thread;
            use std::time::Duration;
            thread::sleep(Duration::from_millis(50));
            unsafe {
                use windows::Win32::UI::Input::KeyboardAndMouse::*;
                let inputs = [
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x11), ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x56), ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x56), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
                    INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0x11), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
                ];
                SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            info!("[Mock Shortcut] Simulated Ctrl+V paste");
        }
    }
}
