# KeyMind — AI Copilot Palette Implementation Plan
### Feature: `Ctrl+Space` Floating Prompt Palette

---

## Overview

This document covers the full implementation of the AI Copilot Palette — a system-wide floating prompt bar triggered by `Ctrl+Space`. It grabs surrounding text context from the active text field, sends a free-form prompt + context to the Groq/Cerebras LLM, and injects the result back into the text field (or clipboard if no field is focused).

The feature spans four layers: Win32 system integration, Rust daemon logic, Tauri IPC bridge, and React frontend.

---

## Architecture

```
Ctrl+Space keypress
        │
        ▼
[keymind-interceptor-windows]
  RegisterHotKey(Ctrl+Space)
  HotKey fires → spawn palette task
        │
        ▼
[keymind-palette] (new crate)
  1. Capture context via IUIAutomation
  2. Spawn Tauri palette window (borderless, topmost)
  3. Send context to frontend via Tauri event
        │
        ▼
[keymind-control-center] (React)
  Render palette UI
  User types prompt → hits Enter
  Emit prompt + context via Tauri command
        │
        ▼
[keymind-sync-server]
  Build system prompt + context + user prompt
  POST to Groq API (Cerebras fallback)
  Stream or return result
        │
        ▼
[keymind-palette]
  Result received
  If text field active → inject via SendInput / IUIAutomation
  If no text field   → write to clipboard via arboard
        │
        ▼
  Close palette window
```

---

## Phase 1 — Win32 Hotkey Registration

**Crate**: `keymind-interceptor-windows`

**Task**: Register `Ctrl+Space` as a global hotkey using `RegisterHotKey`.

```rust
// In the interceptor's message loop thread
RegisterHotKey(
    HWND(0),           // not tied to a window
    HOTKEY_ID_PALETTE, // unique ID e.g. 0x0001
    MOD_CONTROL,       // modifier: Ctrl
    VK_SPACE as u32,   // key: Space
);
```

The existing `GetMessageW` / `DispatchMessageW` loop already runs on a dedicated thread. Add a `WM_HOTKEY` branch to handle the new message:

```rust
WM_HOTKEY => {
    if wparam.0 == HOTKEY_ID_PALETTE as usize {
        // spawn palette task (non-blocking)
        tokio::spawn(open_palette());
    }
}
```

**Notes**:
- `RegisterHotKey` with `HWND(0)` posts `WM_HOTKEY` to the thread's message queue, not a window — this is correct since the interceptor already runs its own message loop.
- `Ctrl+Space` conflicts with some IME input methods. Add a config flag in `app_preferences` to let users remap it.
- Unregister on daemon shutdown: `UnregisterHotKey(HWND(0), HOTKEY_ID_PALETTE)`.

---

## Phase 2 — Context Capture

**Crate**: `keymind-palette` (new)

**Task**: Before opening the palette, read the text surrounding the cursor in the currently focused text field.

### Method: IUIAutomation (preferred)

```rust
use windows::Win32::UI::Accessibility::{
    IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
    TextPatternRangeEndpoint_Start, TextPatternRangeEndpoint_End,
};

pub fn capture_context() -> Option<CapturedContext> {
    unsafe {
        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;

        let focused = automation.GetFocusedElement().ok()?;
        let text_pattern: IUIAutomationTextPattern =
            focused.GetCurrentPattern(UIA_TextPatternId).ok()?.cast().ok()?;

        // Get the full document range
        let doc_range = text_pattern.DocumentRange().ok()?;

        // Get cursor position (degenerate selection = caret)
        let selection = text_pattern.GetSelection().ok()?;
        let caret_range = selection.GetElement(0).ok()?;

        // Expand ~250 chars before and after caret
        let before_range = doc_range.Clone().ok()?;
        before_range.MoveEndpointByRange(
            TextPatternRangeEndpoint_End,
            &caret_range,
            TextPatternRangeEndpoint_Start,
        ).ok()?;

        let after_range = caret_range.Clone().ok()?;
        // Move end forward ~250 chars
        after_range.ExpandToEnclosingUnit(/* TextUnit_Paragraph */);

        let before_text = before_range.GetText(250).ok()?;
        let after_text = after_range.GetText(250).ok()?;

        Some(CapturedContext {
            before: before_text.to_string(),
            after: after_text.to_string(),
            app_name: get_active_window_title(),
            has_text_field: true,
        })
    }
}
```

### Fallback: Clipboard Sniff

If `IUIAutomation` returns nothing (app doesn't expose accessibility tree — e.g. some games or non-standard UIs):

1. Save current clipboard content
2. Simulate `Ctrl+A` → `Ctrl+C` on the focused window
3. Read clipboard
4. Restore original clipboard content
5. Use full clipboard text as context (truncated to 500 chars around cursor — cursor position unknown in this path, so use the last 500 chars)

This fallback is lossy and intrusive — only use it when IUIAutomation returns no result.

### Data Structure

```rust
pub struct CapturedContext {
    pub before: String,       // text before cursor, up to 250 chars
    pub after: String,        // text after cursor, up to 250 chars
    pub app_name: String,     // active window title e.g. "Visual Studio Code"
    pub has_text_field: bool, // false = result goes to clipboard
}
```

---

## Phase 3 — Palette Window (Tauri)

**Crate**: `keymind-control-center` (Tauri app)

### 3a. Create a Dedicated Palette Window

In `tauri.conf.json`, define a second window:

```json
{
  "windows": [
    {
      "label": "main",
      "title": "KeyMind",
      ...
    },
    {
      "label": "palette",
      "title": "",
      "url": "palette.html",
      "width": 560,
      "height": 120,
      "resizable": false,
      "decorations": false,
      "alwaysOnTop": true,
      "visible": false,
      "center": false,
      "skipTaskbar": true,
      "focus": true
    }
  ]
}
```

Key flags:
- `decorations: false` — no OS titlebar or frame, palette draws its own rounded border
- `alwaysOnTop: true` — stays above all other windows
- `visible: false` — hidden at startup, shown programmatically
- `skipTaskbar: true` — doesn't appear in Windows taskbar

### 3b. Open the Palette from Rust

```rust
pub async fn open_palette(app: tauri::AppHandle, context: CapturedContext) {
    let palette = app.get_window("palette").unwrap();

    // Position: centered horizontally, 38% from top of primary monitor
    let monitor = palette.current_monitor().unwrap().unwrap();
    let size = monitor.size();
    let x = (size.width as i32 / 2) - 280; // 280 = half of 560px
    let y = (size.height as f64 * 0.38) as i32;
    palette.set_position(tauri::PhysicalPosition::new(x, y)).unwrap();

    // Send context to frontend before showing
    palette.emit("palette-context", &context).unwrap();

    // Show and focus
    palette.show().unwrap();
    palette.set_focus().unwrap();
}
```

### 3c. Close the Palette

```rust
#[tauri::command]
pub fn close_palette(app: tauri::AppHandle) {
    if let Some(palette) = app.get_window("palette") {
        palette.hide().unwrap();
    }
}
```

Hide rather than destroy — re-showing is instant, re-creating has startup cost.

---

## Phase 4 — Frontend (React)

**File**: `src/palette/Palette.tsx`

### State Machine

```typescript
type PaletteState =
  | { status: 'idle' }
  | { status: 'loading'; prompt: string }
  | { status: 'result'; text: string }
  | { status: 'error'; message: string };
```

### Component Structure

```tsx
export function Palette() {
  const [state, setState] = useState<PaletteState>({ status: 'idle' });
  const [context, setContext] = useState<CapturedContext | null>(null);
  const [prompt, setPrompt] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  // Receive context from Rust when palette opens
  useEffect(() => {
    const unlisten = listen<CapturedContext>('palette-context', (event) => {
      setContext(event.payload);
      setState({ status: 'idle' });
      setPrompt('');
      inputRef.current?.focus();
    });
    return () => { unlisten.then(f => f()); };
  }, []);

  // Esc to close
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (state.status === 'loading') cancelRequest();
        invoke('close_palette');
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [state]);

  const submit = async () => {
    if (!prompt.trim()) return;
    setState({ status: 'loading', prompt });
    try {
      const result = await invoke<string>('run_copilot_prompt', {
        prompt,
        contextBefore: context?.before ?? '',
        contextAfter: context?.after ?? '',
      });
      setState({ status: 'result', text: result });
    } catch (err) {
      setState({ status: 'error', message: String(err) });
    }
  };

  const insert = async () => {
    if (state.status !== 'result') return;
    await invoke('inject_text', { text: state.text });
    invoke('close_palette');
  };

  const copy = async () => {
    if (state.status !== 'result') return;
    await invoke('copy_to_clipboard', { text: state.text });
    invoke('close_palette');
  };

  // render ...
}
```

### Keyboard Handling

| Key | State | Action |
|---|---|---|
| `Esc` | any | Cancel / close |
| `Enter` | idle | Submit prompt |
| `Enter` | result | Insert (if text field active) |
| `Tab` | result | Copy to clipboard |

---

## Phase 5 — Tauri Commands (IPC)

**File**: `src-tauri/src/commands/palette.rs`

```rust
#[tauri::command]
pub async fn run_copilot_prompt(
    prompt: String,
    context_before: String,
    context_after: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let system_prompt = format!(
        "You are a typing assistant embedded in a desktop app. \
         The user is editing text and needs your help. \
         Reply with ONLY the requested output — no preamble, \
         no explanation, no quotation marks."
    );

    let user_message = format!(
        "Context (text surrounding the cursor):\n\
         [...] {} [CURSOR] {} [...]\n\n\
         Task: {}",
        context_before.trim(),
        context_after.trim(),
        prompt
    );

    // Try Groq first
    match state.groq_client.complete(&system_prompt, &user_message).await {
        Ok(result) => Ok(result),
        Err(e) if e.is_rate_limit() || e.is_server_error() => {
            // Failover to Cerebras
            state.cerebras_client
                .complete(&system_prompt, &user_message)
                .await
                .map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn inject_text(text: String) -> Result<(), String> {
    // Use SendInput to type the result at the current cursor position
    // The cursor is still where it was when Ctrl+Space fired
    // because the palette window is HWND_TOPMOST but didn't steal
    // focus from the text field in the same way
    //
    // Strategy: hide palette first, restore focus to previous window,
    // then SendInput the text character by character via VK_ codes
    inject_via_send_input(&text).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn copy_to_clipboard(text: String) -> Result<(), String> {
    use arboard::Clipboard;
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())
}
```

---

## Phase 6 — Text Injection

**File**: `keymind-palette/src/inject.rs`

Injecting text at the cursor is the trickiest part. The palette is `HWND_TOPMOST` and focused — the original text field has lost keyboard focus. The sequence:

```rust
pub fn inject_via_send_input(text: &str) -> windows::core::Result<()> {
    unsafe {
        // 1. Hide the palette window (this returns focus to the previous window)
        //    The Tauri window hide is called from Rust before this fn
        //    Give Windows 50ms to process the focus change
        std::thread::sleep(std::time::Duration::from_millis(50));

        // 2. Build INPUT array — one keydown+keyup per character
        //    Use KEYEVENTF_UNICODE for full Unicode support
        let mut inputs: Vec<INPUT> = Vec::new();
        for ch in text.chars() {
            let scan = ch as u16;
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: scan,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: scan,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }

        // 3. Send all inputs in one batch
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}
```

**Known limitation**: `SendInput` with `KEYEVENTF_UNICODE` works in most apps (VS Code, Notepad, Slack, Chrome, Word). Some apps (terminal emulators, games) intercept at a lower level — in those cases the text lands in whatever field is focused after the palette closes, which may not be the original. This is acceptable for v1; document it.

**Alternative for future**: IUIAutomation `IUIAutomationValuePattern::SetValue` — sets the field value directly without simulating keypresses. More reliable but overwrites the entire field value (need to reconstruct: before + result + after text). Implement as Phase 2 refinement.

---

## Phase 7 — Window Focus Tracking

**Problem**: When the user presses `Ctrl+Space`, the palette must remember which window was previously focused so it can restore focus before injecting text.

```rust
// In keymind-palette, before opening the palette window:
pub fn get_focused_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

// Store it:
pub struct PaletteSession {
    pub previous_hwnd: HWND,
    pub context: CapturedContext,
}

// Before injecting text:
pub fn restore_focus(hwnd: HWND) {
    unsafe {
        SetForegroundWindow(hwnd);
        // Small delay to ensure focus is restored
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
```

---

## Phase 8 — System Prompt Engineering

The quality of the palette's output depends entirely on the system prompt. This is the v1 system prompt:

```
You are a typing assistant embedded in a desktop productivity app called KeyMind.
The user is actively editing text in another application and has asked for your help.

Rules:
- Reply with ONLY the requested output. No preamble, no explanation, no "Here is...", no quotation marks.
- If the user asks to rewrite or improve text, output the rewritten text only.
- If the user asks a question, answer it directly and concisely.
- If the user asks to continue text, output only the continuation (do not repeat what came before).
- Match the tone and style of the surrounding context unless the user asks you to change it.
- Keep responses concise. The user is typing in a text field, not reading an essay.
- Never add markdown formatting (no **, no #, no ---) unless the surrounding context already uses it.
```

---

## Phase 9 — New Database Column

Add to `app_preferences` table to support per-app palette disable:

```sql
ALTER TABLE app_preferences
ADD COLUMN ai_palette_enabled BOOLEAN DEFAULT 1;
```

Add to `user_variables` / settings table:

```sql
CREATE TABLE IF NOT EXISTS palette_settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    hotkey TEXT DEFAULT 'Ctrl+Space',
    context_window_chars INTEGER DEFAULT 500,
    model_preference TEXT DEFAULT 'groq'  -- 'groq' | 'cerebras'
);
```

---

## Implementation Order

| Phase | Task | Complexity | Dependencies |
|---|---|---|---|
| 1 | `RegisterHotKey` for Ctrl+Space | Low | existing interceptor thread |
| 2 | `IUIAutomation` context capture | Medium | windows-rs accessibility APIs |
| 3 | Tauri palette window creation | Low | Tauri multiwindow config |
| 4 | React palette UI + state machine | Medium | Tauri events, invoke |
| 5 | `run_copilot_prompt` Tauri command | Low | existing keymind-sync-server |
| 6 | `SendInput` text injection | Medium | focus tracking (Phase 7) |
| 7 | Focus tracking & restore | Low | Win32 GetForegroundWindow |
| 8 | System prompt tuning | Low | none |
| 9 | DB schema additions | Low | existing SQLite pool |

Recommended order: 1 → 3 → 4 → 5 → 7 → 6 → 2 → 8 → 9

Start with the hotkey + dummy palette window + mock context so the UI is testable immediately, then layer in real context capture and injection.

---

## New Files & Crates

```
keymind-palette/               ← new crate
  src/
    lib.rs
    context.rs                 ← IUIAutomation capture
    inject.rs                  ← SendInput injection
    focus.rs                   ← foreground window tracking
    window.rs                  ← Tauri palette window management

src-tauri/src/commands/
  palette.rs                   ← run_copilot_prompt, inject_text, copy_to_clipboard

src/palette/
  Palette.tsx                  ← main palette React component
  palette.html                 ← dedicated HTML entry point for palette window
  PaletteContext.tsx           ← context strip subcomponent
  PaletteInput.tsx             ← input row subcomponent
  PaletteResult.tsx            ← result + action buttons subcomponent
  palette.css                  ← scoped styles
```

---

## Cargo.toml Additions

```toml
# keymind-palette/Cargo.toml
[dependencies]
windows = { version = "0.52", features = [
    "Win32_UI_Accessibility",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_System_Com",
    "Win32_Foundation",
] }
tokio = { version = "1", features = ["full"] }
tauri = { version = "1", features = [] }
arboard = "3"
serde = { version = "1", features = ["derive"] }
```

---

## Edge Cases & Known Issues

| Scenario | Handling |
|---|---|
| App doesn't expose accessibility tree | Clipboard fallback (Phase 2 fallback) |
| User types very long result | Palette height grows to max 260px, then scrolls result area |
| Groq and Cerebras both fail | Error state shown, user can retry or close |
| `Ctrl+Space` fires inside KeyMind Control Center itself | Ignore — detect `GetForegroundWindow` == palette or control center HWND and no-op |
| User presses `Ctrl+Space` while palette is already open | No-op — palette is already visible |
| Multi-monitor setup | Palette opens on the monitor containing the active window, not always primary |
| IME conflict (CJK input methods) | Add remapping option in Settings; document known conflict |
| Very slow LLM response (> 10s) | Show elapsed time counter in loading state after 3s |

---

## Phase 7 — Verification, State Locking & Build Artifact Distribution

### 7.1 Verification & State Locking
* **Single-Channel Event Loop**: The global interceptor lifecycle (`lifecycle.rs`) maintains an unbuffered channel to route hotkey triggers directly without dropped events or UI freezes.
* **Backend Mutex Lock Stabilization**: All Tauri IPC command handlers (including palette queries and shortcut rebinds) utilize thread-safe `Arc<Mutex<T>>` locking, verified across 43 audit check points.

### 7.2 Release Artifacts & Distribution Packaging
KeyMind and the Copilot Palette feature are compiled and packaged into dual release formats:

1. **Standalone Installer**: `KeyStroke_Installer_v0.1.0.exe`
   - Configured with `downloadBootstrapper` for seamless Microsoft WebView2 runtime installation.
   - Installs system-wide keyboard hook service and Control Center dashboard.
2. **Portable Executable Package**: `KeyStroke_v0.1.0_Portable_x64.zip`
   - Pre-packaged zero-install x64 distribution containing all compiled Rust binaries, SQLite dictionary database schemas, and frontend bundles.

