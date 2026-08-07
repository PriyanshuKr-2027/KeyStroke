use std::sync::atomic::{AtomicBool, Ordering};

/// Global injection-in-progress guard.
/// Set to `true` while synthetic SendInput events are being fired so the
/// low_level_keyboard_proc hook skips those keystrokes and avoids double-buffering.
pub(crate) static IS_INJECTING: AtomicBool = AtomicBool::new(false);

/// Text and backspace injector for Windows using SendInput.
#[derive(Clone, Default)]
pub struct TextInjector;

impl TextInjector {
    pub fn new() -> Self {
        Self
    }

    /// Inject text using SendInput with KEYEVENTF_UNICODE.
    /// The IS_INJECTING guard is set for the entire duration so the hook ignores these events.
    pub fn inject_text(&self, text: &str) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
            };

            IS_INJECTING.store(true, Ordering::SeqCst);

            for c in text.chars() {
                let mut utf16_buf = [0u16; 2];
                let utf16 = c.encode_utf16(&mut utf16_buf);

                for &code_unit in utf16.iter() {
                    let mut inputs = [
                        INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: 0,
                                    wScan: code_unit,
                                    dwFlags: KEYEVENTF_UNICODE,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        },
                        INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: 0,
                                    wScan: code_unit,
                                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        },
                    ];

                    unsafe {
                        SendInput(2, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
                    }
                }
            }

            IS_INJECTING.store(false, Ordering::SeqCst);
        }

        #[cfg(not(target_os = "windows"))]
        {
            tracing::info!("[Mock Windows Injector] Injected text: \"{}\"", text);
        }
    }

    /// Send `n` backspace key events using SendInput VK_BACK (0x08).
    /// IS_INJECTING is set so the hook does not pop those backspaces off WORD_BUFFER
    /// (the buffer is cleared by clear_word_buffer() from main.rs instead).
    /// A 25 ms pause after all backspaces ensures the OS finishes processing them
    /// before the replacement text arrives, preventing the doubled-first-letter race.
    pub fn send_backspaces(&self, n: usize) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_BACK,
            };

            IS_INJECTING.store(true, Ordering::SeqCst);

            for _ in 0..n {
                let mut inputs = [
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_BACK,
                                wScan: 0,
                                dwFlags: 0,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK_BACK,
                                wScan: 0,
                                dwFlags: KEYEVENTF_KEYUP,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                ];

                unsafe {
                    SendInput(2, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
                }
            }

            // Critical: Wait for OS to flush all backspace events before injecting
            // replacement text. Without this pause, the hook can see the first
            // injected character BEFORE all backspaces have been processed, causing
            // the doubled-first-letter bug (Bug 4).
            std::thread::sleep(std::time::Duration::from_millis(25));

            IS_INJECTING.store(false, Ordering::SeqCst);
        }

        #[cfg(not(target_os = "windows"))]
        {
            tracing::info!("[Mock Windows Injector] Sent {} backspaces", n);
        }
    }

    /// Simulate Ctrl+C copy event
    pub fn simulate_copy(&self) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_C, VK_CONTROL,
            };

            let mut inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_CONTROL,
                            wScan: 0,
                            dwFlags: 0,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_C,
                            wScan: 0,
                            dwFlags: 0,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_C,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_CONTROL,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            unsafe {
                SendInput(4, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        #[cfg(not(target_os = "windows"))]
        {
            tracing::info!("[Mock Injector] Simulated Ctrl+C copy");
        }
    }

    /// Simulate Ctrl+V paste event
    pub fn simulate_paste(&self) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_V,
            };

            let mut inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_CONTROL,
                            wScan: 0,
                            dwFlags: 0,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_V,
                            wScan: 0,
                            dwFlags: 0,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_V,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_CONTROL,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            unsafe {
                SendInput(4, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            tracing::info!("[Mock Injector] Simulated Ctrl+V paste");
        }
    }
}
