# KeyStroke — Product Requirements Document (PRD)

## 1. Product Overview & Vision

**KeyStroke** is a system-wide, AI-powered desktop typing intelligence suite for Windows and macOS. Operating quietly in the background via low-level keyboard interception (`WH_KEYBOARD_LL` on Windows, `CGEventTap` on macOS), KeyStroke provides real-time autocorrect, sub-1ms next-word prediction, multi-layered sentence grammar checking, custom snippet expansions, and dual-provider AI copilot assistance across all native desktop applications (VS Code, Slack, Word, Browsers, Terminals).

### Core Vision Statement
> "To transform system-wide text entry into a frictionless, intelligent experience where every keypress is instantly enhanced, corrected, and expanded with zero latency, zero false positives, and 100% data privacy."

---

## 2. Problem Statement & Target Audience

### The Problem
1. **Typing Friction & Aggressive Autocorrect**: Legacy spellcheckers mangle valid English words, force bad corrections on specialized vocabulary, and re-correct mistakes even after the user backspaces and retypes.
2. **Context Switching**: Users interrupt their focus to copy-paste text into browser-based AI or grammar tools for rewriting.
3. **Repetitive Text Entry**: Frequently typed snippets (emails, dates, code templates, clipboard contents) require manual copying.
4. **Elevated Window Blocks (UIPI)**: Unsigned keyboard utilities fail to function inside elevated windows (Administrator terminals, Task Manager).
5. **Memory Bloat**: Electron and Chromium-based typing tools consume hundreds of megabytes of system RAM.

### Target Personas
* **Software Engineers & Technical Power Users**: Require fast slash-command text expansion (`/email`, `/date`), custom technical jargon whitelisting, and zero input delay in IDEs.
* **Executives & Managers**: Require formal business tone rewrites (`Ctrl+Alt+P`), rapid selection summarization (`Ctrl+Alt+S`), and real-time grammar checks in Slack, Outlook, and Teams.
* **Content Writers & Editors**: Require homophone resolution (`their` vs `there`), dictionary memory, and non-intrusive prediction chip tooltips.

---

## 3. Product Goals & Success Metrics

| Goal | Success Metric (KPI) | Target SLA |
| :--- | :--- | :--- |
| **Typo Elimination** | Autocorrect accuracy (Zero False Positives) | > 99.5% accuracy via Google-10k Layer 0 whitelist & distance=1 limit |
| **User Intent Respect** | Rejection memory | 100% suppression of rejected corrections upon user backspace + retype |
| **Typing Velocity Boost** | Words-per-minute improvement | +30% increase via next-word prediction & `/` snippet expansions |
| **System Performance** | Keypress interception latency | P99 < 1ms delay per key event |
| **UIPI Bypass** | Elevated window compatibility | Full function in admin windows via `uiAccess=true` & Program Files installation |
| **AI Availability** | Copilot response availability | 99.9% uptime via Groq $\rightarrow$ Cerebras dual failover |
| **Memory Footprint** | System RAM utilization | < 50 MB total RAM via WebView2 OS working set trimming |

---

## 4. Functional Requirements Summary

### Module 1: System-Wide Interceptor & Pause Control
* Must capture low-level keyboard events across Windows (`WH_KEYBOARD_LL`) and macOS (`CGEventTap`).
* Must provide instant real-time ON/OFF pause control via sidebar toggle button and global hotkey (`Ctrl+Alt+K`).
* Must set `IS_INJECTING` atomic flags with 25ms post-backspace delay to prevent synthetic SendInput events from double-buffering or doubling first letters (`iinitial`).

### Module 2: Multi-Layered Autocorrect & Rejection Memory
* **Layer 0 (Google-10k Whitelist)**: Embedded `HashSet` of top 4,758 most frequent English words. Words in this list are never auto-corrected.
* **Layer 1 (Short-Word Gate)**: Words $\le$ 3 characters pass through untouched.
* **Layer 2 (SymSpell + QWERTY Spatial Error Model)**: Max dictionary edit distance set strictly to 1. Adjacent QWERTY typos require 2.5$\times$ frequency threshold; non-adjacent typos require 10.0$\times$.
* **Rejection Memory**: Session `user_correction_overrides` set. If autocorrect replaces word $W$ and the user backspaces to retype $W$, autocorrect is permanently skipped for $W$ in that session.

### Module 3: Next-Word Prediction Engine
* Must calculate trigram probabilities and next-token candidate lists.
* Must render a floating Gboard-style prediction chip accepting suggestions with <kbd>Tab ↹</kbd>.

### Module 4: Multi-Layered Grammar Engine
* Must require $\ge$ 4 whitespace-delimited words and sentence-ending punctuation (`.`, `!`, `?`) before invoking `nlprule` grammar checks, preventing single-word mangling.
* Must apply fixes in descending character offset order (right-to-left) to preserve text indexing.

### Module 5: Text Shortcuts & Variable Expansion
* Must trigger exclusively when typed words start with `/` (e.g. `/email`, `/date`, `/address`).
* Must support multi-placeholder dynamic expansion (`{date}`, `{time}`, `{clipboard}`).
* Must execute `clear_word_buffer()` immediately after injection to ensure the word buffer resets cleanly.

### Module 6: Global Hotkeys & Automated Actions
* Must support 6 global hotkeys:
  * `Ctrl+Alt+Space`: Open AI Copilot Palette
  * `Ctrl+Alt+G`: Grammar Fix Highlighted Selection
  * `Ctrl+Alt+P`: Professional Rewrite Selection
  * `Ctrl+Alt+S`: Summarize Selection
  * `Ctrl+Alt+X`: Expand AI Prompt Selection
  * `Ctrl+Alt+K`: Toggle Interceptor ON/OFF
* Actions must automatically capture selected text via `Ctrl+C`, run the AI/Grammar pipeline, and paste the result back via `Ctrl+V`.

### Module 7: Claude Light & Dark UI Theme System
* Must provide dual curated themes: **Claude Light** (`#FAF8F5` paper background) and **Claude Dark** (`#1B1917` charcoal background).
* Must use clear, non-developer terminology (Notion-style, no jargon like "SymSpell").

### Module 8: Windows Enterprise Deployment & Code Signing
* Must bundle Windows application manifest (`keystroke.manifest`) with `uiAccess="true"` for UIPI bypass.
* Must configure Tauri installer to target `perMachine` installation (`C:\Program Files\KeyStroke`).
* Must integrate SignPath GitHub Actions workflow (`release.yml`) for automated code-signing.

---

## 5. Non-Functional Requirements

* **Privacy & Security**: All autocorrect, dictionary, and text shortcut processing executes 100% locally on the device. API keys stored locally in JSON configuration.
* **Cross-Platform Compatibility**: Core Rust modules compile cleanly for Windows (`x86_64-pc-windows-msvc`) and macOS (`x86_64` / `aarch64`).
* **Resource Optimization**: Background `SetProcessWorkingSetSize` trimmer runs every 30 seconds to flush unused memory back to the OS.
