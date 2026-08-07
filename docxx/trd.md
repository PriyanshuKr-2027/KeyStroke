# KeyStroke — Technical Requirements Document (TRD)

## 1. System Architecture Overview

KeyStroke is structured as a multi-crate Rust workspace integrated with a Tauri frontend:

```
[ Windows WH_KEYBOARD_LL / macOS CGEventTap ]
                     │
                     ▼ (Raw VK Events)
     [ keymind-interceptor-windows ]
                     │
                     ▼ (WordCompleted / Event Channel)
        [ keymind-control-center ] (Tauri Core)
       /             │             \
      ▼              ▼              ▼
[Autocorrect]   [Grammar Engine]  [AI Copilot]
(SymSpell+10k)    (nlprule)      (Groq/Cerebras)
```

---

## 2. Component Specifications

### 2.1 Low-Level Keyboard Interceptor (`keymind-interceptor-windows`)
* **Hook Mechanism**: `SetWindowsHookExW(WH_KEYBOARD_LL, low_level_keyboard_proc, ...)` running inside a dedicated Win32 message pump thread (`GetMessageW`).
* **Pause / Resume State**: Managed via static `INTERCEPTOR_ACTIVE: AtomicBool`. When `false`, `low_level_keyboard_proc` returns `CallNextHookEx` immediately.
* **Synthetic Event Protection**: Static `IS_INJECTING: AtomicBool` guard. When `true`, hook skips buffer processing. Includes a 25ms post-backspace delay in `send_backspaces` to flush OS event queues before injecting replacement characters.
* **Buffer Management**: `WORD_BUFFER` stores characters typed between whitespace delimiters (`' '`, `'\t'`, `'\r'`, `'\n'`). Public `clear_word_buffer()` function allows the main engine to reset the buffer after every text injection.

### 2.2 Autocorrect Engine (`keymind-autocorrect`)
* **Layer 0 (Google-10k Whitelist)**: Embedded `google-10000-english-no-swears.txt` parsed into a static `OnceLock<HashSet<&'static str>>`. O(1) hash check bypasses autocorrect entirely for common words (`issue`, `grammar`, `this`, `from`, etc.).
* **Layer 1 (Short Word Gate)**: Words $\le 3$ characters are skipped.
* **Layer 2 (SymSpell Engine)**: `SymSpellBuilder` initialized with `max_dictionary_edit_distance(1)` and `prefix_length(7)` over an 82k unigram frequency dictionary. Unified single lookup with `Verbosity::Closest` at distance 1.
* **Spatial QWERTY Matrix**: `is_qwerty_typo()` calculates single-substitution key adjacency. Adjacent typos require $2.5\times$ candidate frequency threshold; non-adjacent typos require $10.0\times$.

### 2.3 Rejection Memory (`AppState`)
* **Structure**: `user_correction_overrides: Mutex<HashSet<String>>` stored in global `AppState`.
* **Flow**:
  1. When autocorrect replaces word $W$ with $S$, $W.to_lowercase()$ is inserted into `user_correction_overrides`.
  2. On subsequent `WordCompleted` events for $W$, `is_user_rejected` evaluates to `true`, skipping autocorrect entirely.

### 2.4 Text Shortcuts & Variables Engine
* **Trigger Constraint**: Triggers strictly when `word.starts_with('/')`.
* **Matching**: Strips the leading `/` and searches `store.variables` against `/key`, `key`, or `clean_key`.
* **Placeholders**: Dynamically evaluates `{date}` (`%B %d, %Y`), `{time}` (`%H:%M:%S`), and `{clipboard}` (via `arboard`).
* **Injection**: Sends `word.len() + 1` backspaces, injects replacement text, and executes `clear_word_buffer()`.

### 2.5 Global Hotkeys & Automated Actions
* **Registration**: Win32 `RegisterHotKey` calls in message loop thread.
* **Event Dispatch**: Async event loop receives `Event::HotKeyTriggered(id)` and invokes `handle_shortcut_trigger(id).await`.
* **Actions**:
  * `"copilot_palette"`: Awaits `open_palette_window(app_handle).await`.
  * `"copilot_summarize"` / `"copilot_professional"` / `"ai_expand"` / `"grammar_fix"`: Simulates `Ctrl+C`, reads selected text from clipboard, calls `run_copilot_prompt` or `GrammarEngine`, updates clipboard with result, and simulates `Ctrl+V`.

---

## 3. UI Design System & RAM Optimization

### 3.1 Claude Theme Design System
* **Claude Light**: Paper background (`#FAF8F5`), white card surfaces (`#FFFFFF`), warm border (`#E8E4DC`), dark primary text (`#1E1E1E`), terracotta accent (`#DA7756`).
* **Claude Dark**: Charcoal background (`#1B1917`), dark card surfaces (`#22201D`), border (`#383430`), light primary text (`#ECE9E3`), terracotta accent (`#DA7756`).

### 3.2 WebView2 Memory Optimization
* **Chromium Flags**: `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` set to `--disable-gpu --disable-gpu-compositing --renderer-process-limit=1 --js-flags="--max-old-space-size=48"`.
* **OS Heap Trimming**: Background thread invokes `SetProcessWorkingSetSize(GetCurrentProcess(), usize::MAX, usize::MAX)` every 30 seconds to return unallocated heap memory to Windows OS.

---

## 4. Windows Deployment & Code Signing

* **Manifest**: `keystroke.manifest` with `uiAccess="true"` and `asInvoker` privileges.
* **Resource Embedding**: `keystroke.rc` compiled into `src-tauri` binary via `build.rs` using `embed-resource`.
* **Installation Target**: `tauri.conf.json` configured with `"installMode": "perMachine"` targeting `C:\Program Files\KeyStroke\`.
* **SignPath CI/CD**: `release.yml` GitHub Actions workflow configured with `signpath/github-action-submit-signing-request@v1` using `SIGNPATH_API_TOKEN` and `SIGNPATH_ORGANIZATION_ID` secrets.
