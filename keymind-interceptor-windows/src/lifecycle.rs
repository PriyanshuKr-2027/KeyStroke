use crate::events::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::info;

pub struct HookHandle {
    is_running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
    watchdog_handle: Option<JoinHandle<()>>,
}

impl HookHandle {
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn stop(mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        if let Some(h) = self.thread_handle.take() {
            let _ = h.join();
        }
        if let Some(w) = self.watchdog_handle.take() {
            let _ = w.join();
        }
        info!("Windows Interceptor hook stopped gracefully.");
    }
}

pub const HOTKEY_ID_PALETTE: i32 = 0x0001;

pub fn update_registered_hotkey(id: i32, modifiers: u32, vk_code: u32) {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey};
        unsafe {
            UnregisterHotKey(0, id);
            RegisterHotKey(0, id, modifiers, vk_code);
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (id, modifiers, vk_code);
    }
}

pub fn start_interceptor(sender: mpsc::Sender<Event>) -> HookHandle {
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = is_running.clone();

    let thread_handle = thread::spawn(move || {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_SHIFT, VK_SPACE,
            };
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY,
            };

            unsafe {
                // Register default global hotkeys
                RegisterHotKey(0, 1, (MOD_CONTROL | MOD_ALT) as u32, VK_SPACE as u32);
                RegisterHotKey(0, 2, (MOD_CONTROL | MOD_SHIFT) as u32, 0x47); // Ctrl+Shift+G (grammar)
                RegisterHotKey(0, 3, (MOD_CONTROL | MOD_SHIFT) as u32, 0x50); // Ctrl+Shift+P (pro)
                RegisterHotKey(0, 4, (MOD_CONTROL | MOD_SHIFT) as u32, 0x53); // Ctrl+Shift+S (summarize)
                RegisterHotKey(0, 5, (MOD_CONTROL | MOD_SHIFT) as u32, 0x45); // Ctrl+Shift+E (expand)
                RegisterHotKey(0, 6, (MOD_CONTROL | MOD_SHIFT) as u32, 0x4B); // Ctrl+Shift+K (toggle)
            }

            while is_running_clone.load(Ordering::SeqCst) {
                unsafe {
                    let mut msg: MSG = std::mem::zeroed();
                    while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                        if !is_running_clone.load(Ordering::SeqCst) {
                            break;
                        }

                        if msg.message == WM_HOTKEY {
                            let id = msg.wParam as u32;
                            if id == 1 {
                                let _ = sender.blocking_send(Event::PaletteRequested);
                            } else {
                                let _ = sender.blocking_send(Event::HotKeyTriggered(id));
                            }
                        }

                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }

            unsafe {
                for id in 1..=6 {
                    UnregisterHotKey(0, id);
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            while is_running_clone.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    // Watchdog thread monitoring hook health
    let is_running_watchdog = is_running.clone();
    let watchdog_handle = thread::spawn(move || {
        while is_running_watchdog.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
            // Watchdog heartbeat check
        }
    });

    HookHandle {
        is_running,
        thread_handle: Some(thread_handle),
        watchdog_handle: Some(watchdog_handle),
    }
}
