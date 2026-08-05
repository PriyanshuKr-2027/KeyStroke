/// Text and backspace injector for Windows using SendInput.
#[derive(Clone, Default)]
pub struct TextInjector;

impl TextInjector {
    pub fn new() -> Self {
        Self
    }

    /// Inject text using SendInput with KEYEVENTF_UNICODE.
    pub fn inject_text(&self, text: &str) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
            };

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
        }

        #[cfg(not(target_os = "windows"))]
        {
            tracing::info!("[Mock Windows Injector] Injected text: \"{}\"", text);
        }
    }

    /// Send `n` backspace key events using SendInput VK_BACK (0x08).
    pub fn send_backspaces(&self, n: usize) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_BACK,
            };

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
