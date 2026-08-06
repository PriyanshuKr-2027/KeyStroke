# KeyMind — Product Requirements Document (PRD)

## 1. Product Overview & Vision

**KeyMind** is a system-wide, AI-powered desktop typing intelligence suite for Windows and macOS. Operating quietly in the background via low-level keyboard interception, KeyMind provides real-time autocorrect, sub-1ms next-word prediction, multi-layered grammar checking, custom snippet expansions, and dual-provider AI copilot assistance across all native applications (VS Code, Slack, Word, Browsers, Terminals).

### Core Vision Statement
> "To transform system-wide text entry into a frictionless, intelligent experience where every keypress is instantly enhanced, corrected, and expanded with zero latency and 100% data privacy."

---

## 2. Problem Statement & Target Audience

### The Problem
1. **Typing Friction & Errors**: Users waste significant time fixing typos, grammar mistakes, and homophone confusions across different desktop applications.
2. **Context Switching**: Users switch back and forth between their active text editor and browser-based AI or grammar tools to rewrite text.
3. **Repetitive Text Entry**: Frequently typed text (emails, dates, code templates, addresses) requires manual copy-pasting.
4. **Privacy Concerns**: Cloud-first keyloggers and typing assistants risk exposing sensitive passwords, personal data, and corporate intellectual property.

### Target Personas
* **Software Engineers & Technical Power Users**: Require fast snippet expansion (`/trigger`), custom technical jargon whitelisting, and zero input delay in IDEs.
* **Executives & Managers**: Require formal business tone rewrites, rapid email drafting, and real-time grammar checks in Slack, Outlook, and Teams.
* **Content Writers & Editors**: Require homophone resolution (`their` vs `there`), vocabulary phrase memory, and non-intrusive suggestion tooltips.

---

## 3. Product Goals & Success Metrics

| Goal | Success Metric (KPI) | Target SLA |
| :--- | :--- | :--- |
| **Typo Elimination** | Automatic correction accuracy | > 98% accuracy on top 82k English words |
| **Typing Velocity Boost** | Words-per-minute improvement | +25% increase via next-word prediction & snippets |
| **System Performance** | Keypress interception latency | P99 < 1ms delay per key event |
| **Grammar Engine Quality** | Correct right-to-left text fixes | Zero character offset corruption during multi-fix passes |
| **AI Availability** | Copilot response availability | 99.9% uptime via Groq $\rightarrow$ Cerebras dual failover |
| **Memory Footprint** | System RAM utilization | < 80 MB total RAM across background daemon and UI |

---

## 4. User Personas & Detailed Use Cases

### Persona A: Alex (Senior Software Engineer)
* **Goal**: Expand boilerplate code and dates without leaving VS Code or Terminal.
* **Flow**: Types `/date` followed by `Space`. KeyMind instantly intercepts the trigger and expands it into `August 5, 2026`.
* **Value**: Zero interruption to engineering flow; custom technical jargon (e.g. `SymSpell`, `SQLite`) is never flagged as typos.

### Persona B: Sarah (Product Manager)
* **Goal**: Write grammatically flawless project updates in Slack and Notion.
* **Flow**: Types *"He are going to teh store because there books was lost."* KeyMind evaluates the sentence in real-time and updates it to *"He is going to the store because their books were lost."*
* **Value**: Professional communication without manual proofreading.

---

## 5. Functional Requirements Summary

### Module 1: System-Wide Key Interceptor
* Must capture low-level keyboard events across Windows (`WH_KEYBOARD_LL`) and macOS (`CGEventTap`).
* Must route events through an unbuffered lifecycle channel connected directly to the core event handler loop.
* Must support instant passthrough with sub-1ms overhead.

### Module 2: SymSpell Autocorrect & Homophone Resolution
* Must execute fast edit distance lookup (max distance: 2) against an 82,000-word frequency dictionary.
* Must resolve context-dependent homophones (`their` vs `there` vs `they're`, `then` vs `than`).

### Module 3: Next-Word Prediction Engine
* Must calculate trigram probabilities and transformer predictions for the current typing context.
* Must render a floating Gboard-style prediction chip accepting suggestions with <kbd>Tab ↹</kbd>.

### Module 4: Multi-Layered Grammar Engine
* Must query LanguageTool server endpoints for subject-verb, punctuation, and style checks.
* Must apply fixes in descending character offset order (right-to-left) to preserve text indexing.

### Module 5: Snippets & Variable Expansion Engine
* Must monitor for slash command triggers (`/trigger`).
* Must expand static text, dynamic computed expressions (`/date`, `/time`), and AI prompt outputs (`/reply`).

### Module 6: Per-App Rules & Blacklist
* Must detect the active application window bundle identifier (e.g. `com.microsoft.VSCode`).
* Must allow users to toggle Autocorrect, Grammar, or AI Copilot on a per-app basis.

### Module 7: Global Hotkeys & Shortcuts
* Must register and map 6 system-wide global hotkeys (`Ctrl+Alt+Space` for Autocorrect toggle, `Ctrl+Alt+G` for Grammar, `Ctrl+Alt+P` for Prediction, `Ctrl+Alt+M` for Menu, `Ctrl+Alt+S` for Snippet, `Ctrl+Alt+W` for Window focus).
* Must feature interactive keypress recording in the Control Center UI with instant shortcut updates.

### Module 8: First-Run Onboarding & System Accessibility Setup
* Must guide new users through permission setup, AI key configuration, and typing presets.
* Must include direct OS accessibility launcher trigger (`open_accessibility_settings`) and auto-advance/continue state handling.

---

## 6. Non-Functional Requirements

* **Privacy & Security**: All autocorrect, prediction, and dictionary operations must process 100% locally on the device. API keys must be encrypted in OS keychain storage.
* **Cross-Platform Compatibility**: Rust engine core must compile cleanly for Windows (`x86_64-pc-windows-msvc` / `gnu`) and macOS (`x86_64` / `aarch64`).
* **Installer & Deployment**: Windows installer pipeline must bundle WebView2 bootstrapper (`downloadBootstrapper`) for seamless single-click setup across client systems.
* **Reliability & Thread Safety**: Foreground application and IPC handlers must remain fully responsive and panic-free, protecting backend state locks under heavy concurrent events.

