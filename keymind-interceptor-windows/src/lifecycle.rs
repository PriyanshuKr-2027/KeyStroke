use crate::events::Event;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::info;

pub struct HookHandle {
    is_running: Arc<AtomicBool>,
    thread_id: Arc<AtomicU32>,
    thread_handle: Option<JoinHandle<()>>,
    watchdog_handle: Option<JoinHandle<()>>,
}

impl HookHandle {
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn stop(mut self) {
        self.is_running.store(false, Ordering::SeqCst);

        // Post WM_QUIT to unblock GetMessageW on the hook thread
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::WindowsAndMessaging::PostThreadMessageW;
            let tid = self.thread_id.load(Ordering::SeqCst);
            if tid != 0 {
                unsafe {
                    PostThreadMessageW(tid, 0x0012 /* WM_QUIT */, 0, 0);
                }
            }
        }

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
static SENDER: parking_lot::Mutex<Option<mpsc::Sender<Event>>> = parking_lot::const_mutex(None);
#[cfg(target_os = "windows")]
static WORD_BUFFER: parking_lot::Mutex<String> = parking_lot::const_mutex(String::new());

/// Global ON/OFF interceptor pause flag.
/// When set to false, low_level_keyboard_proc ignores all keystrokes and passes them through.
pub(crate) static INTERCEPTOR_ACTIVE: AtomicBool = AtomicBool::new(true);

pub fn set_interceptor_active(active: bool) {
    INTERCEPTOR_ACTIVE.store(active, Ordering::SeqCst);
    info!("Keyboard interceptor active state set to: {}", active);
}

pub fn is_interceptor_active() -> bool {
    INTERCEPTOR_ACTIVE.load(Ordering::SeqCst)
}

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

    // Skip processing if interceptor is turned OFF or if synthetic injection is in progress
    if !INTERCEPTOR_ACTIVE.load(Ordering::SeqCst)
        || crate::injector::IS_INJECTING.load(std::sync::atomic::Ordering::SeqCst)
    {
        return CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param);
    }

    if n_code >= 0 && (w_param == WM_KEYDOWN as usize || w_param == WM_SYSKEYDOWN as usize) {
        let kbd = *(l_param as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode;

        if let Some(ch) = crate::hook::translate_vk_code(vk, kbd.scanCode) {
            let is_control = (GetAsyncKeyState(VK_CONTROL as i32) as u16 & 0x8000) != 0;
            let is_alt = (GetAsyncKeyState(VK_MENU as i32) as u16 & 0x8000) != 0;

            if is_control && is_alt {
                if let Some(ref sender) = *SENDER.lock() {
                    let handled = match vk {
                        0x20 => { let _ = sender.try_send(Event::PaletteRequested); true }
                        0x47 => { let _ = sender.try_send(Event::HotKeyTriggered(2)); true }
                        0x50 => { let _ = sender.try_send(Event::HotKeyTriggered(3)); true }
                        0x53 => { let _ = sender.try_send(Event::HotKeyTriggered(4)); true }
                        0x58 => { let _ = sender.try_send(Event::HotKeyTriggered(5)); true }
                        0x4B => { let _ = sender.try_send(Event::HotKeyTriggered(6)); true }
                        _ => false,
                    };
                    if handled {
                        return 1;
                    }
                }
            } else if !is_control && !is_alt {
                // CRITICAL: Use try_lock to avoid blocking — hook must return fast
                if let Some(mut buf) = WORD_BUFFER.try_lock() {
                    if ch.is_alphanumeric() || ch == '/' || ch == '_' || ch == '-' {
                        buf.push(ch);
                    } else if ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t' || ch == '.' || ch == ',' || ch == '!' || ch == '?' {
                        if !buf.is_empty() {
                            let word = buf.clone();
                            buf.clear();
                            drop(buf); // Release lock before trying sender

                            // Non-blocking try_send — never block in hook callback
                            if let Some(ref sender) = *SENDER.lock() {
                                let _ = sender.try_send(Event::WordCompleted {
                                    word: word.clone(),
                                    context: word,
                                });
                            }
                        }
                    }
                }
            }
        } else if vk == VK_BACK as u32 {
            if let Some(mut buf) = WORD_BUFFER.try_lock() {
                let _ = buf.pop();
            }
        }
    }

    CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param)
}

/// Clear the word buffer — called from main.rs immediately after every autocorrect/variable
/// injection so the next word the user types starts from a clean slate (Bug 3 fix).
/// Without this, the buffer still contains the old typed word after correction and the
/// user's next backspace+retype cannot trigger a fresh WordCompleted event.
pub fn clear_word_buffer() {
    #[cfg(target_os = "windows")]
    {
        if let Some(mut buf) = WORD_BUFFER.try_lock() {
            buf.clear();
        }
    }
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
    let _is_running_clone = is_running.clone();
    let thread_id_store = Arc::new(AtomicU32::new(0));
    let thread_id_clone = thread_id_store.clone();

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
            use windows_sys::Win32::System::Threading::GetCurrentThreadId;

            unsafe {
                // Store this thread's ID so stop() can post WM_QUIT to it
                let tid = GetCurrentThreadId();
                thread_id_clone.store(tid, Ordering::SeqCst);

                *SENDER.lock() = Some(sender.clone());

                // Install WH_KEYBOARD_LL low-level keyboard hook
                let h_instance = GetModuleHandleW(std::ptr::null());
                HOOK_HANDLE = SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(low_level_keyboard_proc),
                    h_instance,
                    0,
                );

                // Register default global hotkeys
                RegisterHotKey(0, 1, (MOD_CONTROL | MOD_ALT) as u32, VK_SPACE as u32);
                RegisterHotKey(0, 2, (MOD_CONTROL | MOD_ALT) as u32, 0x47); // Ctrl+Alt+G
                RegisterHotKey(0, 3, (MOD_CONTROL | MOD_ALT) as u32, 0x50); // Ctrl+Alt+P
                RegisterHotKey(0, 4, (MOD_CONTROL | MOD_ALT) as u32, 0x53); // Ctrl+Alt+S
                RegisterHotKey(0, 5, (MOD_CONTROL | MOD_ALT) as u32, 0x58); // Ctrl+Alt+X
                RegisterHotKey(0, 6, (MOD_CONTROL | MOD_ALT) as u32, 0x4B); // Ctrl+Alt+K
            }

            // Message pump — GetMessageW blocks until a message arrives or WM_QUIT
            loop {
                unsafe {
                    let mut msg: MSG = std::mem::zeroed();
                    let ret = GetMessageW(&mut msg, 0, 0, 0);
                    if ret <= 0 {
                        // ret == 0 means WM_QUIT received, ret == -1 means error
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
        thread_id: thread_id_store,
        thread_handle: Some(thread_handle),
        watchdog_handle: Some(watchdog_handle),
    }
}
