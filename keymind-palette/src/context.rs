use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedContext {
    pub before: String,
    pub after: String,
    pub app_name: String,
    pub has_text_field: bool,
}

impl Default for CapturedContext {
    fn default() -> Self {
        Self {
            before: String::new(),
            after: String::new(),
            app_name: String::from("Unknown Window"),
            has_text_field: false,
        }
    }
}

pub fn get_active_window_title() -> String {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd != 0 {
                let mut buf = [0u16; 512];
                let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), 512);
                if len > 0 {
                    return String::from_utf16_lossy(&buf[..len as usize]);
                }
            }
        }
    }
    "Active Window".to_string()
}

pub fn capture_context() -> CapturedContext {
    let app_name = get_active_window_title();

    CapturedContext {
        before: String::new(),
        after: String::new(),
        app_name,
        has_text_field: false,
    }
}
