use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedContext {
    pub before: String,
    pub after: String,
    pub app_name: String,
    pub has_text_field: bool,
    pub caret_x: Option<i32>,
    pub caret_y: Option<i32>,
}

impl Default for CapturedContext {
    fn default() -> Self {
        Self {
            before: String::new(),
            after: String::new(),
            app_name: String::from("Unknown Window"),
            has_text_field: false,
            caret_x: None,
            caret_y: None,
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

pub fn get_caret_position() -> (Option<i32>, Option<i32>) {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            GetGUIThreadInfo, GUITHREADINFO,
        };
        use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
        use windows_sys::Win32::Foundation::POINT;

        unsafe {
            let mut info: GUITHREADINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<GUITHREADINFO>() as u32;
            if GetGUIThreadInfo(0, &mut info) != 0 && info.hwndCaret != 0 {
                let mut pt = POINT {
                    x: info.rcCaret.left,
                    y: info.rcCaret.bottom + 8,
                };
                ClientToScreen(info.hwndCaret, &mut pt);
                return (Some(pt.x), Some(pt.y));
            }
        }
    }
    (None, None)
}

pub fn capture_context() -> CapturedContext {
    let app_name = get_active_window_title();
    let (caret_x, caret_y) = get_caret_position();

    CapturedContext {
        before: String::new(),
        after: String::new(),
        app_name,
        has_text_field: false,
        caret_x,
        caret_y,
    }
}
