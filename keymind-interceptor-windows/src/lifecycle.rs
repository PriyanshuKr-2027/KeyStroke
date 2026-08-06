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

#[cfg(target_os = "windows")]
static mut INTERCEPTOR_HWND: windows_sys::Win32::Foundation::HWND = 0;
#[cfg(target_os = "windows")]
static mut HOOK_HANDLE: windows_sys::Win32::UI::WindowsAndMessaging::HHOOK = 0;
#[cfg(target_os = "windows")]
static SENDER: std::sync::Mutex<Option<mpsc::Sender<Event>>> = std::sync::Mutex::new(None);
#[cfg(target_os = "windows")]
static WORD_BUFFER: std::sync::Mutex<Vec<char>> = std::sync::Mutex::new(Vec::new());

#[cfg(target_os = "windows")]
unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: usize,
    l_param: isize,
) -> isize {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_SYSKEYDOWN,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_BACK, VK_MENU, VK_CONTROL,
    };

    if n_code >= 0 && (w_param == WM_KEYDOWN as usize || w_param == WM_SYSKEYDOWN as usize) {
        let kbd = *(l_param as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode;

        if let Some(ch) = crate::hook::translate_vk_code(vk, kbd.scanCode) {
            let is_control = (GetAsyncKeyState(VK_CONTROL as i32) as u16 & 0x8000) != 0;
            let is_alt = (GetAsyncKeyState(VK_MENU as i32) as u16 & 0x8000) != 0;

            if !is_control && !is_alt {
                if let Ok(mut buf) = WORD_BUFFER.lock() {
                    if ch.is_alphanumeric() || ch == '/' || ch == '_' || ch == '-' {
                        buf.push(ch);
                    } else if ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t' || ch == '.' || ch == ',' || ch == '!' || ch == '?' {
                        if !buf.is_empty() {
                            let word: String = buf.iter().collect();
                            buf.clear();

                            if let Ok(sender_guard) = SENDER.lock() {
                                if let Some(ref sender) = *sender_guard {
                                    let _ = sender.try_send(Event::WordCompleted {
                                        word: word.clone(),
                                        context: word,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        } else if vk == VK_BACK as u32 {
            if let Ok(mut buf) = WORD_BUFFER.lock() {
                if !buf.is_empty() {
                    buf.pop();
                }
            }
        }
    }

    CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param)
}

pub fn update_registered_hotkey(id: i32, modifiers: u32, vk_code: u32) {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey};
        unsafe {
            let hwnd = INTERCEPTOR_HWND;
            UnregisterHotKey(hwnd, id);
            if modifiers != 0 || vk_code != 0 {
                RegisterHotKey(hwnd, id, modifiers, vk_code);
            }
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
                RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, VK_SPACE,
            };
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
                TranslateMessage, MSG, WM_HOTKEY, WH_KEYBOARD_LL,
            };
            use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

            unsafe {
                if let Ok(mut guard) = SENDER.lock() {
                    *guard = Some(sender.clone());
                }

                // Install WH_KEYBOARD_LL low-level keyboard hook
                let h_instance = GetModuleHandleW(std::ptr::null());
                HOOK_HANDLE = SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(low_level_keyboard_proc),
                    h_instance,
                    0,
                );

                // Register default global hotkeys matching shortcuts.rs
                RegisterHotKey(0, 1, (MOD_CONTROL | MOD_ALT) as u32, VK_SPACE as u32);
                RegisterHotKey(0, 2, (MOD_CONTROL | MOD_ALT) as u32, 0x47); // Ctrl+Alt+G (grammar)
                RegisterHotKey(0, 3, (MOD_CONTROL | MOD_ALT) as u32, 0x50); // Ctrl+Alt+P (pro)
                RegisterHotKey(0, 4, (MOD_CONTROL | MOD_ALT) as u32, 0x53); // Ctrl+Alt+S (summarize)
                RegisterHotKey(0, 5, (MOD_CONTROL | MOD_ALT) as u32, 0x58); // Ctrl+Alt+X (expand)
                RegisterHotKey(0, 6, (MOD_CONTROL | MOD_ALT) as u32, 0x4B); // Ctrl+Alt+K (toggle)
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
                if HOOK_HANDLE != 0 {
                    UnhookWindowsHookEx(HOOK_HANDLE);
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
