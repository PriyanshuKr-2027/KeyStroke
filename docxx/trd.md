# KeyMind — Technical Requirements Document (TRD)

## 1. System Architecture Diagram

KeyMind is architected as a modular Rust workspace composed of low-level OS native hooks, high-performance C/C++ bindings, SQLite data storage, ONNX machine learning runtime, and a Tauri/React desktop interface.

```
+-----------------------------------------------------------------------------------+
|                            TAURI / REACT FRONTEND UI                              |
|   (Sidebar Navigation, Dashboard Metrics, Variables Manager, Grammar Sandbox)     |
+-----------------------------------------------------------------------------------+
                                         │  IPC Bridge (Tauri Command / JSON-RPC)
                                         ▼
+-----------------------------------------------------------------------------------+
|                                KEYMIND CORE DAEMON                                |
|                                (keymind-engine)                                   |
+-----------------------------------------------------------------------------------+
       │                         │                        │                     │
       ▼                         ▼                        ▼                     ▼
[Key Interceptor]       [Autocorrect Engine]    [Prediction Engine]   [Grammar Engine]
keymind-interceptor     keymind-autocorrect     keymind-prediction    keymind-grammar
(Win32 Hook /           (SymSpell 0.5.2 +       (Trigram Model +      (LanguageTool
 CGEventTap)             SQLite Dictionary)      ONNX Runtime)         HTTP Server Client)
                                 │                                              │
                                 ▼                                              ▼
                         [Local Storage]                              [Dual AI Failover]
                         SQLite DB Pool                               Groq + Cerebras
                         (dictionary.db)                              Llama 3.3 70B
```

---

## 2. Crate & Subsystem Specifications

### 2.1 `keymind-interceptor-windows`
* **Technology**: Win32 API (`windows-sys`, `user32.dll`).
* **Hook Mechanism**: `SetWindowsHookExW` registering a `WH_KEYBOARD_LL` (low-level keyboard) callback procedure with direct event-handler channel loop integration (`lifecycle.rs`).
* **Threading**: Dedicated thread running a standard Win32 message loop (`GetMessageW` / `DispatchMessageW`) to prevent OS input freezes.
* **Global Hotkeys**: Maps all 6 system-wide hotkeys (`Ctrl+Alt+Space` for Autocorrect, `Ctrl+Alt+G` for Grammar, `Ctrl+Alt+P` for Prediction, `Ctrl+Alt+M` for Menu, `Ctrl+Alt+S` for Snippet, `Ctrl+Alt+W` for Window focus).
* **Latency SLA**: < 1ms per keypress event.

### 2.2 `keymind-autocorrect`
* **Technology**: `symspell 0.5.2`, `sqlx` / `rusqlite`, SQLite 3.
* **Dictionary Data**: `frequency_dictionary_en_82k.txt` indexed into an in-memory SymSpell hash map.
* **Database Pool**: Thread-safe `Arc<SqlitePool>` accessing `keymind-autocorrect/data/dictionary.db`.
* **Homophone Resolution**: Pattern match maps for homophone pairs (`their`/`there`/`they're`, `then`/`than`, `your`/`you're`).
* **Dictionary Hygiene**: Strict validation requiring typo entries (`teh`, `recieve`) to be omitted from valid vocabulary dictionaries so SymSpell lookup triggers replacement logic.

### 2.3 `keymind-prediction`
* **Technology**: Trigram model structure, ONNX Runtime (`ort` C++ bindings), `tokenizers`.
* **Inference Pipeline**: Evaluates 2-word typing context (`context = "how are"`), calculates conditional probabilities against n-gram dictionary, and outputs top candidate predictions (e.g. `you`, `there`).
* **Memory Optimization**: Model weights loaded via `mmap` to minimize RAM consumption (< 25MB).

### 2.4 `keymind-grammar`
* **Technology**: `reqwest`, `serde_json`, `arboard` (Clipboard management).
* **Right-to-Left Replacement Algorithm**:
  ```rust
  // Sort detected grammar issues by start offset in DESCENDING order
  issues.sort_by(|a, b| b.offset.cmp(&a.offset));
  for issue in issues {
      let start = issue.offset;
      let end = start + issue.length;
      result.replace_range(start..end, &issue.replacement);
  }
  ```
  *This guarantees that replacing text at the end of a string does not shift character indices for earlier errors.*
* **HTTP Client**: Connects to local or remote LanguageTool server (`http://127.0.0.1:8081/v2/check`) with automatic retry and timeout handling.

### 2.5 `keymind-variables`
* **Technology**: Dynamic evaluation engine.
* **Resolution Pipeline**:
  - `Static`: Evaluates raw string literal replacement.
  - `Dynamic`: Evaluates runtime expressions (`/date` $\rightarrow$ `Local::now()`, `/time`, `/clipboard`).
  - `AI`: Dispatches system prompt + context to AI provider client.

### 2.6 `keymind-sync-server` (Dual AI Copilot Client)
* **Primary Provider**: **Groq API** (`llama-3.3-70b-versatile`) for ultra-fast response (< 150ms).
* **Failover Provider**: **Cerebras API** (`llama3.1-8b`) automatically triggered if Groq returns HTTP 429 (Rate Limit) or HTTP 5xx errors.

### 2.7 `keymind-control-center` (Tauri Desktop App & IPC Bridge)
* **Technology**: Tauri 1.x / 2.x, React 18, TypeScript, Tailwind CSS.
* **IPC Command Bridge**: Exposes system commands including `open_accessibility_settings`, live shortcut recording, per-app whitelist/blacklist toggles, and dictionary state manipulation.
* **Thread Safety**: Mutex lock stabilization across all backend state handlers prevents deadlocks and panics under rapid IPC calls.

---

## 3. Database Schema & Data Models

Database schema stored in SQLite (`keymind-autocorrect/data/dictionary.db`):

```sql
-- Frequency Dictionary Table
CREATE TABLE IF NOT EXISTS frequency_dictionary (
    word TEXT PRIMARY KEY,
    count INTEGER NOT NULL
);

-- Custom User Whitelist
CREATE TABLE IF NOT EXISTS custom_whitelist (
    id TEXT PRIMARY KEY,
    word TEXT UNIQUE NOT NULL,
    date_added TEXT NOT NULL
);

-- Learned Frequent Phrases
CREATE TABLE IF NOT EXISTS learned_phrases (
    id TEXT PRIMARY KEY,
    phrase TEXT UNIQUE NOT NULL,
    frequency INTEGER DEFAULT 1,
    is_pinned BOOLEAN DEFAULT 0,
    app_name TEXT
);

-- Snippets & Variables
CREATE TABLE IF NOT EXISTS user_variables (
    key TEXT PRIMARY KEY,
    var_type TEXT NOT NULL, -- 'static', 'dynamic', 'ai'
    value TEXT,
    ai_prompt TEXT,
    description TEXT,
    use_count INTEGER DEFAULT 0
);

-- Per-App Preferences
CREATE TABLE IF NOT EXISTS app_preferences (
    app_bundle_id TEXT PRIMARY KEY,
    app_name TEXT NOT NULL,
    autocorrect_enabled BOOLEAN DEFAULT 1,
    grammar_enabled BOOLEAN DEFAULT 1,
    ai_copilot_enabled BOOLEAN DEFAULT 1,
    is_blocked BOOLEAN DEFAULT 0
);
```

---

## 4. Build System & Toolchain Setup

* **Compiler Toolchain**: Rust `1.85+` target `x86_64-pc-windows-msvc` or `x86_64-pc-windows-gnu`.
* **Linker Resolution**: Modern 64-bit MinGW-w64 (`winlibs-x86_64-posix-seh-gcc-16.1.0`) placed at the top of System `PATH` to resolve GNU `dlltool.exe` conflicts during `windows-sys` and `libsqlite3-sys` compilation.
* **Bundling & Installer**: Windows `downloadBootstrapper` configured for WebView2 runtime installer bundling in NSIS / MSI installers.
* **Frontend Build**: Vite 5 + React 18 + Tailwind CSS 3.4 (`postcss.config.js` and `tailwind.config.js` configured for PostCSS pipeline).
