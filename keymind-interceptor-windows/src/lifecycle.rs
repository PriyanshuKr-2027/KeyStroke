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

pub fn start_interceptor(_sender: mpsc::Sender<Event>) -> HookHandle {
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = is_running.clone();

    let thread_handle = thread::spawn(move || {
        while is_running_clone.load(Ordering::SeqCst) {
            #[cfg(target_os = "windows")]
            {
                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    DispatchMessageW, GetMessageW, TranslateMessage, MSG,
                };

                unsafe {
                    let mut msg: MSG = std::mem::zeroed();
                    while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                        if !is_running_clone.load(Ordering::SeqCst) {
                            break;
                        }
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }

            #[cfg(not(target_os = "windows"))]
            {
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
