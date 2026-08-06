# KeyStroke — Pipeline Architecture & Broken Code Snippets Breakdown

**Document Version**: 1.0  
**Project**: KeyStroke Desktop Typing Assistant  
**Artifact File**: `keystroke_pipelines_and_broken_code_snippets.md`  

---

## Overview

This document provides a comprehensive code-level reference of each core processing pipeline within KeyStroke. For every pipeline, exact code snippets are documented alongside detailed technical breakdowns of the broken parts, edge cases, and runtime failure modes.

---

## Pipeline 1: Windows Low-Level Interceptor & OS Hook Pipeline

### Path: `keymind-interceptor-windows/src/lifecycle.rs` & `injector.rs`

### Pipeline Role
Installs the Windows `WH_KEYBOARD_LL` low-level keyboard hook, listens for global keystrokes and hotkey events, buffers typed characters into completed words, and injects backspaces/text via `SendInput`.

### Code Snippet (`lifecycle.rs`)

```rust
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
```

### Broken / Failing Parts Analysis

1. **Windows UIPI (User Interface Privilege Isolation) Block**:
   - *Broken Part*: Standard un-elevated processes running in Windows cannot intercept or inject keystrokes into elevated windows (Command Prompt as Admin, Task Manager, installer windows, or security dialogs).
   - *Impact*: Low-level hooks fail silently without raising runtime exceptions when focused on elevated applications.

2. **Windows Hook Procedure Timeout (`LowLevelHooksTimeout`)**:
   - *Broken Part*: If `low_level_keyboard_proc` takes longer than the OS registry threshold (default 300ms–1000ms) or if `WORD_BUFFER.lock()` contends with Tokio threads, Windows Defender / OS Kernel automatically unhooks `WH_KEYBOARD_LL`.
   - *Impact*: Interceptor stops receiving keystroke events after a few seconds of typing.

---

## Pipeline 2: Tauri Backend Event Dispatcher & Main Loop Pipeline

### Path: `keymind-control-center/src-tauri/src/main.rs`

### Pipeline Role
Listens on the MPSC channel for `Event::WordCompleted`, `Event::HotKeyTriggered`, and `Event::PaletteRequested` emitted by the Windows interceptor, then executes text replacement or palette window launch.

### Code Snippet (`main.rs`)

```rust
tauri::Builder::default()
    .setup(|app| {
        #[cfg(target_os = "windows")]
        {
            let (tx, mut rx) = tokio::sync::mpsc::channel(64);
            std::thread::spawn(move || {
                let _ = keymind_interceptor_windows::lifecycle::start_interceptor(tx);
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        keymind_interceptor_windows::Event::PaletteRequested => {
                            let _ = open_palette_window(app_handle.clone());
                        }
                        keymind_interceptor_windows::Event::HotKeyTriggered(id) => {
                            let shortcut_name = match id {
                                1 => "copilot_palette",
                                2 => "grammar_fix",
                                3 => "copilot_professional",
                                4 => "copilot_summarize",
                                5 => "ai_expand",
                                6 => "toggle_engine",
                                _ => "",
                            };
                            if shortcut_name == "copilot_palette" {
                                let _ = open_palette_window(app_handle.clone());
                            } else if !shortcut_name.is_empty() {
                                let _ = handle_shortcut_trigger(shortcut_name.to_string()).await;
                            }
                        }
                        keymind_interceptor_windows::Event::WordCompleted { word, context: _ } => {
                            let state = app_handle.state::<AppState>();
                            let store = state.store.lock().unwrap_or_else(|e| e.into_inner());

                            // 1. Check custom variables (e.g. /email -> user@example.com)
                            let replacement = store.variables.iter().find(|v| v.key.eq_ignore_ascii_case(&word)).and_then(|v| v.value.as_deref());

                            if let Some(rep) = replacement {
                                let injector = keymind_interceptor_windows::TextInjector::new();
                                injector.send_backspaces(word.len());
                                injector.inject_text(rep);
                            } else {
                                // 2. Check static autocorrect dictionary
                                let static_autocorrect: std::collections::HashMap<&str, &str> = [
                                    ("teh", "the"),
                                    ("recieve", "receive"),
                                    ("seperate", "separate"),
                                    ("occured", "occurred"),
                                    ("untill", "until"),
                                    ("waht", "what"),
                                    ("htat", "that"),
                                    ("thier", "their"),
                                    ("definately", "definitely"),
                                ].into_iter().collect();

                                if let Some(&corrected) = static_autocorrect.get(word.to_lowercase().as_str()) {
                                    let injector = keymind_interceptor_windows::TextInjector::new();
                                    injector.send_backspaces(word.len());
                                    injector.inject_text(corrected);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            });
        }
        Ok(())
    })
```

### Broken / Failing Parts Analysis

1. **Discarded Channel Receiver (`_rx` bug)**:
   - *Original Broken Code*: `let (tx, _rx) = tokio::sync::mpsc::channel(32);`
   - *Impact*: The `_rx` variable was prefixed with an underscore, causing Rust to drop the receiver immediately. All emitted events were discarded.

2. **Poison Panic Traps (`.lock().unwrap()`)**:
   - *Original Broken Code*: `let store = state.store.lock().unwrap();`
   - *Impact*: If any thread panicked while modifying app settings, the mutex became poisoned, causing all subsequent hotkey calls to crash the application.

---

## Pipeline 3: AI Copilot Palette & Streaming Pipeline

### Path: `keymind-control-center/src-tauri/src/copilot.rs`

### Pipeline Role
Opens the floating AI Copilot window (`palette.html`), streams response chunks from Groq/Cerebras APIs via Server-Sent Events (SSE), and pastes accepted text back into the previously active window.

### Code Snippet (`copilot.rs`)

```rust
#[tauri::command]
pub fn copilot_accept(window: Window, final_text: String) -> Result<(), String> {
    // 1. Write final text to clipboard
    if let Ok(mut cb) = Clipboard::new() {
        if let Err(e) = cb.set_text(final_text) {
            tracing::warn!("Failed to set clipboard text: {}", e);
            return Err("Failed to write to clipboard".to_string());
        }
    } else {
        tracing::warn!("Failed to access clipboard");
        return Err("Failed to write to clipboard".to_string());
    }

    // 2. Hide window and sleep slightly to restore focus to previously active app
    let _ = window.hide();
    std::thread::sleep(std::time::Duration::from_millis(50));

    // 3. Simulate paste in previously active app
    simulate_paste();

    // 4. Close window
    let _ = window.close();
    Ok(())
}
```

### Broken / Failing Parts Analysis

1. **Focus Stealing Paste Bug**:
   - *Original Broken Code*: `simulate_paste(); window.close();` executed BEFORE hiding the window.
   - *Impact*: `Ctrl+V` pasted the generated text back into the palette search box itself instead of the target application (Notepad, Word, Chrome).

2. **UTF-8 Byte Boundary Splitting on SSE Chunks**:
   - *Original Broken Code*: `let text = String::from_utf8_lossy(&chunk);` on raw TCP byte packets.
   - *Impact*: Multi-byte UTF-8 characters split across packet boundaries produced corrupted replacement characters (``).

---

## Pipeline 4: Shortcuts Manager & Modifier Mapping Pipeline

### Path: `keymind-control-center/src-tauri/src/shortcuts.rs`

### Pipeline Role
Manages user-customizable hotkey bindings (`Ctrl+Alt+Space`, `Ctrl+Alt+G`, etc.), parses shortcut strings, updates registered system hotkeys, and simulates copy/paste clipboard triggers.

### Code Snippet (`shortcuts.rs`)

```rust
pub fn parse_shortcut_str(s: &str) -> (u32, u32) {
    let mut mods: u32 = 0;
    let mut vk: u32 = 0;

    for part in s.split(&['+', ' '][..]) {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        match p.to_lowercase().as_str() {
            "ctrl" | "control" => mods |= 0x0002, // MOD_CONTROL
            "alt" | "option" => mods |= 0x0001,   // MOD_ALT
            "shift" => mods |= 0x0004,            // MOD_SHIFT
            "win" | "cmd" | "command" | "meta" => mods |= 0x0008, // MOD_WIN
            "space" => vk = 0x20,
            other => {
                if let Some(ch) = other.chars().next() {
                    if ch.is_ascii_alphanumeric() {
                        vk = ch.to_ascii_uppercase() as u32;
                    }
                }
            }
        }
    }

    (mods, vk)
}
```

### Broken / Failing Parts Analysis

1. **Hotkey Modifier Mismatch**:
   - *Original Broken Code*: `shortcuts.rs` registered `Ctrl+Alt+G` as default, but `lifecycle.rs` initialized `MOD_SHIFT` (`Ctrl+Shift+G`).
   - *Impact*: Pressing `Ctrl+Alt+G` did nothing, while pressing `Ctrl+Shift+G` triggered a shortcut that didn't match the UI display.

2. **Clipboard Read Race Condition**:
   - *Original Broken Code*: Executed `simulate_copy()` and immediately read clipboard text in the same millisecond.
   - *Impact*: Windows clipboard did not have enough time to receive the selected text from external apps, causing empty string returns.

---

## Pipeline 5: KeyMind Engine & Grammar Fixer Pipeline

### Path: `keymind-grammar/src/fixer.rs` & `keymind-variables/src/lib.rs`

### Pipeline Role
Executes SymSpell spelling correction, LanguageTool grammar rule checks, variable replacement, and UTF-8 offset mapping.

### Code Snippet (`keymind-grammar/src/fixer.rs`)

```rust
pub fn char_offset_to_byte_offset(text: &str, char_offset: usize) -> usize {
    text.char_indices()
        .nth(char_offset)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}
```

### Broken / Failing Parts Analysis

1. **Char Index vs Byte Offset Misalignment**:
   - *Original Broken Code*: `text.replace_range(issue.offset..issue.offset + issue.length, replacement);`
   - *Impact*: In non-ASCII or emoji text, LanguageTool returns character offsets. Using character offsets directly as Rust byte offsets caused `replace_range` to slice inside multi-byte UTF-8 characters, panicking the process.

2. **Sync `block_on` Inside Async Tokio Runtime**:
   - *Original Broken Code*: `tokio::runtime::Handle::current().block_on(...)` inside `resolve_ai`.
   - *Impact*: Calling `block_on` from within an already running async Tokio thread panics with `Cannot start a runtime from within a runtime`.

---

## Summary Matrix of Pipeline Failure Points

| Pipeline Name | Primary File | Broken Component | Technical Fix Applied |
| :--- | :--- | :--- | :--- |
| **Windows Interceptor** | `lifecycle.rs` | Missing `WH_KEYBOARD_LL` hook | Implemented native `SetWindowsHookExW` callback & `WORD_BUFFER` Mutex. |
| **Event Dispatcher** | `main.rs` | `_rx` channel receiver dropped | Connected `rx` in `app.setup()` to process `Event::WordCompleted` & hotkeys. |
| **Copilot Palette** | `copilot.rs` | Window focus during paste | Hide window first, 50ms pause, then `simulate_paste()`. |
| **Shortcuts Manager** | `shortcuts.rs` | Modifier mismatch (`Alt` vs `Shift`) | Synchronized modifier bits (`MOD_CONTROL \| MOD_ALT`) across files. |
| **Grammar Engine** | `fixer.rs` | Char/byte offset panic | Added `char_offset_to_byte_offset()` mapping via `char_indices()`. |
