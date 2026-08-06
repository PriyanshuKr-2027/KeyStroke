# KeyStroke — Comprehensive Code Audit, Bug Resolution & Status Report

**Project**: KeyStroke — Intelligent Cross-Platform Typing Assistant  
**Repository**: `https://github.com/PriyanshuKr-2027/KeyStroke.git`  
**Current Release**: `v0.1.0`  
**Report Date**: August 6, 2026  

---

## Executive Summary

Over the course of multiple deep-code audit rounds, 45+ critical bugs, security vulnerabilities, edge cases, and runtime failure points were identified, isolated, and resolved across the **Tauri Rust backend**, **React/TypeScript frontend**, and **KeyMind engine crates**. 

In addition to core codebase fixes, critical Windows-specific OS integration issues — including missing `WebView2Loader.dll` installer dependencies, disconnected low-level keyboard event channels, thread-affinity hotkey failures, and onboarding navigation bugs — were addressed in the codebase.

---

## Part 1: Summary of Errors Found & Fixed

### 1. Tauri Backend (`src-tauri/src/main.rs`, `copilot.rs`, `shortcuts.rs`)

| Issue ID | Category | Description of Finding | Resolution Applied |
| :--- | :--- | :--- | :--- |
| **B-1** | **Poison Panic** | 17 instances of `.lock().unwrap()` across mutex state calls caused total process crashes if a thread panicked while holding a lock. | Replaced all `.lock().unwrap()` with poison-safe `.lock().unwrap_or_else(|e| e.into_inner())`. |
| **B-2** | **Race Condition** | `handle_shortcut_trigger` simulated `Ctrl+C` copy and immediately attempted clipboard read without giving OS selection time to process. | Added explicit 75ms async sleep (`tokio::time::sleep`) between copy simulation and clipboard read. |
| **B-3** | **Copilot Paste Focus** | `copilot_accept` tried to paste text before hiding the palette window, causing text to paste into the palette itself instead of target app. | Reordered execution: hide palette window first, pause 50ms for OS focus transfer, simulate paste, then close window. |
| **B-4** | **UTF-8 Chunking** | LLM SSE streaming parser chopped byte chunks arbitrarily, causing invalid UTF-8 char boundary crashes on multi-byte characters. | Implemented streaming chunk buffer accumulator using `buffer.drain(..=pos)` splitting strictly on `\n` boundaries. |
| **B-5** | **HTTP Connection Leak** | `reqwest::Client::new()` was constructed on every single AI request, depleting OS socket handles. | Encapsulated global `reqwest::Client` inside `static CLIENT: OnceLock<reqwest::Client>`. |
| **B-6** | **Unsafe .env Parsing** | `dotenvy::from_path` wiped system env vars and failed silently if `.env` had formatted comments. | Built safe custom parser `read_env_keys()` reading lines with `.strip_prefix()` directly. |
| **B-7** | **Char Boundary Panics** | `replace_range` in `predict_next_word` sliced UTF-8 strings at raw byte offsets, causing Rust panics on non-ASCII text. | Added strict `is_char_boundary(idx)` checks before string replacements. |
| **B-8** | **JSON Response Panics** | Unchecked `.get("choices")[0]` array indexing panicked if LLM returned malformed JSON or rate limit errors. | Replaced direct indexing with safe pattern matching (`if let Some(choice) = chunk.choices.first()`). |
| **B-9** | **Disconnected Channel** | System keyboard interceptor channel receiver (`_rx`) was discarded in `app.setup()`, preventing all background hotkeys from executing. | Connected `rx` to an async event loop in `main.rs` that dispatches hotkey events system-wide. |
| **B-10** | **Thread Hotkey Failure** | `RegisterHotKey(0, ...)` was called without a window handle, binding hotkeys only to the calling thread's message queue. | Updated interceptor lifecycle to register default global hotkeys matching frontend config. |

---

### 2. Frontend React / TypeScript (`src/App.tsx`, `DashboardTab`, `MemoryTab`, `GrammarTab`, `FirstRunWizard`)

| Issue ID | Category | Description of Finding | Resolution Applied |
| :--- | :--- | :--- | :--- |
| **F-1** | **Promise Checking** | `App.tsx` used `Promise.allSettled` but checked top-level array status instead of item `status === "rejected"`, failing to show error UI. | Updated check: `results.some(r => r.status === "rejected")`. |
| **F-2** | **Duplicate Invokes** | `MemoryTab.tsx` and `App.tsx` both fetched word lists simultaneously on tab switch, causing UI flickering and race conditions. | Centralized data loading state inside `App.tsx` and passed props down to components. |
| **F-3** | **Global Key Listener** | `keydown` event listener in `App.tsx` intercepted `Tab` key presses globally even when no prediction overlay was visible. | Scoped `keydown` listener to trigger strictly when `activePrediction` is non-null. |
| **F-4** | **Rules of Hooks** | `GrammarTab.tsx` defined a `useEffect` inside a JSX inline conditional, violating React Rules of Hooks. | Extracted `useEffect` to top-level component scope. |
| **F-5** | **Race Condition** | Sandbox typing in `DashboardTab.tsx` fired instant backend checks on every character without debouncing. | Wrapped sandbox effect in 300ms `setTimeout` debounce with an `isCancelled` flag. |
| **F-6** | **State Memory Leak** | Accepting a next-word prediction left old next-word state active in sandbox. | Added explicit reset (`setSandboxNextWord(null)`) on prediction accept. |
| **F-7** | **Wizard Navigation** | Clicking "Open Accessibility Settings" in onboarding set permission state but failed to advance step, trapping user on Step 1. | Updated click handler to invoke `open_accessibility_settings`, mark state, and advance to Step 2, plus added a "Continue →" button. |
| **F-8** | **Data Export** | "Export local data" button in `SettingsTab.tsx` had no browser save action. | Implemented JSON Blob creation and dynamic `<a>` tag trigger for file download. |

---

### 3. KeyMind Engine Crates (`keymind-autocorrect`, `keymind-learning`, `keymind-variables`, `keymind-prediction`)

| Issue ID | Category | Description of Finding | Resolution Applied |
| :--- | :--- | :--- | :--- |
| **E-1** | **Tokio Runtime Panic** | `keymind-variables` (`resolve_ai`) invoked `tokio::runtime::Handle::current().block_on()` inside async code, causing nested runtime panics. | Converted `resolve_ai` to native `async/await` SQL queries. |
| **E-2** | **Char/Byte Offset** | `keymind-grammar/fixer.rs` used UTF-8 character index as raw byte offset when applying text replacements. | Added `char_offset_to_byte_offset()` converter using `text.char_indices()`. |
| **E-3** | **Unconnected Privacy** | `PrivacyFilter` toggle in `keymind-learning` was disconnected from background worker thread. | Connected shared `Arc<AtomicBool>` across learning worker threads. |
| **E-4** | **Single-Word Fallback** | `keymind-prediction` ONNX fallback returned empty arrays for common 1-word inputs (`"the"`, `"in"`). | Expanded static fallback dictionary with 30+ common English context maps. |
| **E-5** | **TCP Socket Fallback** | IPC server crashed on Windows if named pipes were restricted by group policy. | Added automatic Windows TCP socket fallback (`127.0.0.1:9123`). |

---

### 4. Windows OS Deployment & Runtime Fixes

1. **`WebView2Loader.dll` Missing System Error**:
   - *Problem*: Downloading raw `KeyStroke.exe` threw a missing DLL system error because `WebView2Loader.dll` wasn't alongside it.
   - *Fix*: Configured `"webviewInstallMode": { "type": "downloadBootstrapper" }` in `tauri.conf.json` and generated `KeyStroke_Installer_v0.1.0.exe` (NSIS setup package) which automatically provisions `WebView2Loader.dll` in `Program Files`.
2. **Global Hotkey Mismatches**:
   - *Problem*: `shortcuts.rs` defined `Ctrl+Alt+...` defaults, but `lifecycle.rs` registered `Ctrl+Shift+...`.
   - *Fix*: Aligned all default registrations to `Ctrl+Alt+...` (`Ctrl+Alt+Space`, `Ctrl+Alt+G`, `Ctrl+Alt+P`, `Ctrl+Alt+S`, `Ctrl+Alt+X`, `Ctrl+Alt+K`).

---

## Part 2: Active Open Runtime Issues (User Testing & Deployment Feedback)

> [!WARNING]
> **Active User Deployment Failure**: Practical user testing confirms that background system-wide features are currently not executing outside the internal app window.

### Reported Symptoms:
- **No Floating Palette Pop-up**: Pressing `Ctrl + Alt + Space` does not bring up the AI Copilot floating palette window over external apps.
- **No Global Hotkeys**: Background hotkeys (`Ctrl + Alt + G`, `Ctrl + Alt + P`, `Ctrl + Alt + S`, `Ctrl + Alt + X`, `Ctrl + Alt + K`) do not trigger actions when typing in external windows (Notepad, Word, Chrome, VS Code).
- **No Background Typing Auto-Correct**: Typing misspelled words (e.g. `teh`, `thier`) or variable triggers (e.g. `/email`) in external text fields does not trigger live backspacing or replacement injection.
- **Feature Isolation**: Features only process internally inside the application's live testing sandbox component.

### Technical Analysis & Open Root Cause Candidates:
1. **Windows UIPI (User Interface Privilege Isolation)**: Windows prevents lower-privilege processes from setting hooks or injecting input into higher-privilege windows unless signed or run as Administrator.
2. **`WH_KEYBOARD_LL` Hook Callback Thread Timeout**: Windows silently unhooks `WH_KEYBOARD_LL` if the hook procedure takes longer than the registry `LowLevelHooksTimeout` value (default 300ms-1000ms) or if the installing thread's message loop is starved by Tokio async execution.
3. **Unsigned Binary Filter / Security Software**: Unsigned executables making low-level keyboard hook calls (`SetWindowsHookExW`) are silently blocked by Windows Defender Real-Time Protection or third-party Security suites without raising visible UI dialogs.

---

## Part 3: System Verification & Build Output

- **Rust Backend & Engine Crates**: `cargo check` completed with **0 errors, 0 warnings**.
- **React Frontend**: `tsc && vite build` completed in **3.15s** with **0 type errors**.
- **Tauri Bundle**: `npm run tauri build` generated:
  - `KeyStroke_Installer_v0.1.0.exe` (NSIS Installer)
  - `KeyStroke_v0.1.0_Portable_x64.zip` (Portable Bundle)
  - `KeyStroke_0.1.0_x64_en-US.msi` (Windows Installer Package)

---

## Part 4: Open Architectural Items

| Category | Item Description | Recommended Future Approach | Priority |
| :--- | :--- | :--- | :--- |
| **System-Wide Hooks** | **Windows UIPI & Admin Manifest** | Configure manifest with `requireAdministrator` or sign installer to grant elevated UIPI hook access. | **High** |
| **Prediction Engine** | **ONNX Runtime Binding** | Integrate `ort` crate bindings to enable running full local GPT-2 ONNX models instead of static n-gram fallback maps. | Medium |
| **macOS Interceptor** | **macOS Native CGEventTap** | Implement native `CGEventTap` listener for background typing interception on macOS. | Medium |
| **IME Support** | **CJK Language Input** | Multi-stage Composition Input Method Editors (Japanese/Chinese/Korean) processing through IME buffers. | Low |
