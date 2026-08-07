# KeyMind — System-Wide AI Desktop Typing Intelligence

> **KeyMind** is a high-performance, system-wide AI desktop typing intelligence suite built in **Rust**, **Tauri**, **React**, and **TypeScript**. Operating quietly in the background via low-level keyboard interception, KeyMind provides real-time QWERTY spatial autocorrect, sub-1ms next-word prediction, multi-pass right-to-left grammar checking, slash command snippet expansion with multi-placeholder support, a floating AI copilot palette, and dual-provider AI assistance across all native applications (IDE, Slack, Browsers, Word, Terminals).

---

## ⚡ Key Features

- **System-Wide Key Interception**: Unbuffered hardware keyboard hooks for Windows (`WH_KEYBOARD_LL`) and macOS (`CGEventTap`) with sub-1ms latency and clean hotkey suppression (`return 1`).
- **Spatial QWERTY Autocorrect & Homophones**: Physical key adjacency proximity model for fat-finger typo likelihood $P(t|w)$ with adaptive scoring (2.5x ratio threshold for adjacent keys vs 10x for non-adjacent), combined with contextual homophone resolution (`their` / `there` / `they're`).
- **Trigram Next-Word Prediction**: Memory-mapped n-gram probabilities & ONNX runtime powering a floating prediction chip accepted with <kbd>Tab ↹</kbd>.
- **Multi-Pass Right-to-Left Grammar Fixer**: Connects to LanguageTool to evaluate full sentence context and applies fixes in descending offset order to avoid character index shifts.
- **In-Line Slash Command Expansions**: Instant trigger expansion for static text, dynamic expressions (`/date`, `/time`), system clipboard injection (`{clipboard}`), multi-placeholder chaining, and exact backspacing.
- **Floating AI Copilot Palette (`keymind-palette`)**: Dedicated quick-access AI prompt bar powered by Groq (`llama-3.3-70b-versatile`) with automatic failover to Cerebras (`llama3.1-8b`).
- **Claude Warm Paper Ivory Aesthetic**: Modern, refined UI for Control Center & Palette using Claude paper ivory (`#FAF8F5`), pure soft white card surfaces (`#FFFFFF`), delicate borders (`#E8E4DC`), and terracotta orange accents (`#DA7756`) with persistent light/dark themes.
- **Ultra-Low Memory Footprint & Background Trimmer**: Optimized WebView2 flags (`--disable-gpu`, `--renderer-process-limit=1`, `--js-flags max-old-space-size=48`) and Win32 heap working-set trimmer saving 120MB+ system RAM for sub-60MB baseline operation.
- **Windows Autostart & Boot Persistence**: Registry-backed auto-start service (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`) for zero-friction background startup on Windows boot.
- **Privacy-First Local Storage**: Custom whitelists and learned user phrases stored 100% locally in SQLite (`dictionary.db`).

---

## 🏗️ System Architecture

```
┌───────────────────────────────────────────────────────────────────────────────────┐
│                      TAURI / REACT CONTROL CENTER & PALETTE                       │
│    (Sidebar, Metrics Dashboard, Variables Manager, Shortcuts, Claude Ivory Theme) │
└───────────────────────────────────────────────────────────────────────────────────┘
                                          │  IPC (Tauri Command / JSON-RPC)
                                          ▼
┌───────────────────────────────────────────────────────────────────────────────────┐
│                                KEYMIND CORE DAEMON                                │
│                   (keymind-engine & Win32 Heap RAM Trimmer)                       │
└───────────────────────────────────────────────────────────────────────────────────┘
       │                         │                        │                     │
       ▼                         ▼                        ▼                     ▼
[Key Interceptor]       [Autocorrect Engine]    [Prediction Engine]   [Grammar Engine]
keymind-interceptor     keymind-autocorrect     keymind-prediction    keymind-grammar
(Win32 Hook /           (QWERTY Spatial +       (Trigram Model +      (LanguageTool
 CGEventTap)             SymSpell 82k Dict)      ONNX Runtime)         HTTP Client)
                                 │                                              │
                                 ▼                                              ▼
                         [Local Storage]                              [Dual AI Failover]
                         SQLite DB Pool                               Groq + Cerebras
                         (dictionary.db)                              Llama 3.3 70B
```

---

## 📦 Workspace Crates

| Crate / Subsystem | Description | Key Technologies |
| :--- | :--- | :--- |
| [`keymind-engine`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-engine) | Main system daemon orchestrating pipelines, RAM trimmer & autostart | Rust, Tokio, Win32 API |
| [`keymind-interceptor-windows`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-interceptor-windows) | Windows low-level keyboard hook & hotkey suppression | Win32 `windows-sys`, `user32.dll` |
| [`keymind-interceptor-macos`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-interceptor-macos) | macOS low-level keyboard event interceptor | CoreGraphics `CGEventTap` |
| [`keymind-autocorrect`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-autocorrect) | QWERTY spatial key proximity model, SymSpell 82k lookup & homophone solver | `symspell`, `rusqlite`, SQLite 3 |
| [`keymind-prediction`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-prediction) | N-gram trigram prediction & ONNX engine | `ort`, `tokenizers`, mmap |
| [`keymind-grammar`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-grammar) | LanguageTool client & right-to-left offset fixer | `reqwest`, `serde_json` |
| [`keymind-variables`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-variables) | Slash trigger expansion engine & multi-placeholder resolver | Dynamic Expression Engine |
| [`keymind-ipc`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-ipc) | Shared SQLite schema, migrations, and IPC protocols | `sqlx`, `tokio` |
| [`keymind-learning`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-learning) | Local user phrase learning & whitelist tracker | SQLite n-gram tracker |
| [`keymind-palette`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-palette) | Dedicated floating AI Copilot prompt bar | Tauri, React, Claude Ivory UI |
| [`keymind-sync-server`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-sync-server) | Dual AI Copilot service & cloud sync API | Express, Prisma, Groq, Cerebras |
| [`keymind-control-center`](file:///c:/Users/10pri/Downloads/KEYBOARD/keymind-control-center) | Desktop dashboard, keybinding recorder & settings manager | Tauri, React, TypeScript, Tailwind |

---

## 🚀 Quick Start

### Prerequisites

- **Rust**: `1.75+` (`cargo`, `rustc`)
- **Node.js**: `v18+` & `npm` / `pnpm`
- **Tauri Prerequisites**: WebView2 (Windows) / WebKitGTK / macOS Command Line Tools

### Build & Run Core Daemon

```bash
# Clone the repository
git clone https://github.com/PriyanshuKr-2027/KeyStroke.git
cd KeyStroke

# Build all workspace crates
cargo build --workspace

# Run interactive engine pipeline test
cargo run --package keymind-engine
```

### Build & Run Control Center UI

```bash
# Navigate to control center directory
cd keymind-control-center

# Install frontend dependencies
npm install

# Run Tauri development server
npm run tauri dev
```

---

## 🧪 Testing

Run the automated test suite across all workspace components:

```bash
# Run unit & integration tests for all crates
cargo test --workspace

# Run autocorrect benchmark
cargo bench --package keymind-autocorrect
```

---

## 💾 Releases & Distribution

KeyMind comes with pre-compiled distribution bundles for Windows 64-bit:
- **Installer**: `KeyStroke_Installer_v0.1.0.exe`
- **Portable**: `KeyStroke_v0.1.0_Portable_x64.zip`

---

## 📄 Documentation

- 📋 [Product Requirements Document (PRD)](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/prd.md)
- ⚙️ [Technical Requirements Document (TRD)](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/trd.md)
- 🎨 [Design Specification](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/design.md)
- 📊 [Feature Matrix & Breakdown](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/features.md)
- 🔍 [Audit & Fix Report](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/keystroke_comprehensive_audit_and_fix_report.md)
- 🚀 [Deployment & Signing Guide](file:///c:/Users/10pri/Downloads/KEYBOARD/docxx/keystroke_deployment_and_signing_guide.md)

---

## ⚖️ License

Distributed under the **MIT OR Apache-2.0** License. See `Cargo.toml` for details.
