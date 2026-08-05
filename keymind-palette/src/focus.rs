#[derive(Debug, Clone, Copy)]
pub struct ActiveWindowHandle {
    pub hwnd: isize,
}

pub fn get_focused_window() -> ActiveWindowHandle {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
        let hwnd = unsafe { GetForegroundWindow() };
        ActiveWindowHandle { hwnd }
    }

    #[cfg(not(target_os = "windows"))]
    {
        ActiveWindowHandle { hwnd: 0 }
    }
}

pub fn restore_focus(handle: ActiveWindowHandle) {
    #[cfg(target_os = "windows")]
    {
        if handle.hwnd != 0 {
            use windows_sys::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
            unsafe {
                let _ = SetForegroundWindow(handle.hwnd);
            }
            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = handle;
    }
}
