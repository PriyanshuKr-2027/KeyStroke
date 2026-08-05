/// Checks if the focused window in Windows has the ES_PASSWORD style or is a password input.
pub fn is_focused_element_secure() -> bool {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetFocus;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            GetClassNameW, GetForegroundWindow, GetWindowLongPtrW, ES_PASSWORD, GWL_STYLE,
        };

        unsafe {
            let hwnd_fore = GetForegroundWindow();
            if hwnd_fore == 0 {
                return false;
            }

            let hwnd_focus = GetFocus();
            let target_hwnd = if hwnd_focus != 0 { hwnd_focus } else { hwnd_fore };

            // 1. Fallback Edit class check with ES_PASSWORD style
            let mut class_buf = [0u16; 64];
            let class_len = GetClassNameW(target_hwnd, class_buf.as_mut_ptr(), 64);
            if class_len > 0 {
                let class_name = String::from_utf16_lossy(&class_buf[..class_len as usize]);
                if class_name.eq_ignore_ascii_case("Edit") {
                    let style = GetWindowLongPtrW(target_hwnd, GWL_STYLE) as u32;
                    if (style & (ES_PASSWORD as u32)) != 0 {
                        return true;
                    }
                }
            }

            false
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

/// Translates Virtual Keycode to Unicode character considering current thread keyboard layout.
pub fn translate_vk_code(vk_code: u32, scan_code: u32) -> Option<char> {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
            GetKeyboardState, ToUnicode,
        };

        unsafe {
            let mut key_state = [0u8; 256];
            if GetKeyboardState(key_state.as_mut_ptr()) == 0 {
                return None;
            }

            let mut out_buf = [0u16; 4];
            let res = ToUnicode(
                vk_code,
                scan_code,
                key_state.as_ptr(),
                out_buf.as_mut_ptr(),
                4,
                0,
            );

            if res > 0 {
                if let Ok(s) = String::from_utf16(&out_buf[..res as usize]) {
                    return s.chars().next();
                }
            }
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Mock translation for non-Windows test environments
        match vk_code {
            0x41 => Some('a'),
            0x42 => Some('b'),
            0x43 => Some('c'),
            0x20 => Some(' '),
            _ => None,
        }
    }
}
