# KeyMind — Feature Specification & Functional Breakdown

## 1. Complete Feature Matrix

| Feature ID | Feature Name | Core Functionality | Status | Subsystem |
| :--- | :--- | :--- | :--- | :--- |
| **FEAT-01** | Low-Level Key Interception | System-wide unbuffered keyboard hook across Windows & macOS | ✅ Verified | `keymind-interceptor-windows` |
| **FEAT-02** | SymSpell Autocorrect | Fast edit distance typo correction against 82k English words | ✅ Verified | `keymind-autocorrect` |
| **FEAT-03** | Homophone Context Swaps | Automatic resolution of `their`/`there`/`they're`, `then`/`than` | ✅ Verified | `keymind-autocorrect` |
| **FEAT-04** | Trigram Next-Word Prediction | Predicts next word based on 2-word typing context | ✅ Verified | `keymind-prediction` |
| **FEAT-05** | Floating Gboard Suggestion Chip | Overlay displaying predicted word with <kbd>Tab ↹</kbd> acceptance | ✅ Verified | `keymind-control-center` |
| **FEAT-06** | LanguageTool Grammar Engine | Real-time right-to-left auto-fixes for subject-verb & typos | ✅ Verified | `keymind-grammar` |
| **FEAT-07** | In-Line Variable Expansions | Slash command expansion (`/email`, `/date`, `/reply`) | ✅ Verified | `keymind-variables` |
| **FEAT-08** | Custom Whitelist Dictionary | Personal dictionary ignoring technical jargon & acronyms | ✅ Verified | `keymind-learning` |
| **FEAT-09** | Learned Frequent Phrases | Auto-learns multi-word phrases ranked by frequency | ✅ Verified | `keymind-learning` |
| **FEAT-10** | Per-App Exclusion Rules | Enable/disable features per application bundle ID | ✅ Verified | `keymind-control-center` |
| **FEAT-11** | Global Keybinding Capture | Custom shortcuts (`Ctrl+Alt+Space`, `Ctrl+Alt+G`) with live recorder | ✅ Verified | `keymind-control-center` |
| **FEAT-12** | Dual AI Copilot Failover | Groq Llama 3.3 70B with automatic Cerebras Llama 3.1 8B failover | ✅ Verified | `keymind-sync-server` |
| **FEAT-13** | First-Run Onboarding Wizard | 3-step setup wizard for permissions, AI keys, & presets | ✅ Verified | `keymind-control-center` |
| **FEAT-14** | Interactive Engine Sandbox | Home & Grammar tab live input bar for instant testing | ✅ Verified | `keymind-control-center` |
| **FEAT-15** | Glassmorphic Desktop Dashboard | Claude Terracotta obsidian UI built with Tauri + React | ✅ Verified | `keymind-control-center` |

---

## 2. Feature Deep-Dives

### FEAT-01: System-Wide Low-Level Key Interception
* **Description**: Captures keypress events across native OS windows before they reach target text fields, enabling instant replacement without input flicker.
* **Input**: Native OS keypress hardware interrupt.
* **Output**: Modified keypress stream or expanded buffer injection.
* **Verification**: `cargo run --example interactive_test` execution.

### FEAT-02 & FEAT-03: SymSpell Autocorrect & Homophones
* **Description**: Corrects transposed characters (e.g. `teh` $\rightarrow$ `the`, `recieve` $\rightarrow$ `receive`) and contextual homophones (e.g. `going over their` $\rightarrow$ `there`).
* **Algorithm**: SymSpell lookup using Delete Edit Distance algorithm with `max_edit_distance = 2`.
* **Dictionary Hygiene**: Typo entries removed from valid vocabulary dictionary text files so SymSpell lookup returns valid target terms.

### FEAT-04 & FEAT-05: Next-Word Prediction & Gboard Floating Chip
* **Description**: Computes next-word probabilities based on typing context. When confidence exceeds threshold (e.g. > 80%), a floating prediction chip appears near the cursor.
* **Interaction**: Pressing <kbd>Tab ↹</kbd> accepts the candidate word and appends a trailing space. Pressing <kbd>Esc</kbd> dismisses the chip.

### FEAT-06: LanguageTool Grammar Engine & Right-to-Left Fixer
* **Description**: Evaluates full sentence context for subject-verb agreement (`He are going` $\rightarrow$ `He is going`), typos (`teh` $\rightarrow$ `the`), homophones (`there books` $\rightarrow$ `their books`), and plural verbs (`books was` $\rightarrow$ `were`).
* **Right-to-Left Execution**:
  ```text
  Input Sentence: "He are going to teh store because there books was lost."
  1. Fix Offset 46: "was"   -> "were"  ("He are going to teh store because there books were lost.")
  2. Fix Offset 34: "there" -> "their" ("He are going to teh store because their books were lost.")
  3. Fix Offset 16: "teh"   -> "the"   ("He are going to the store because their books were lost.")
  4. Fix Offset 3:  "are"   -> "is"    ("He is going to the store because their books were lost.")
  ```

### FEAT-07: In-Line Variable & Snippet Expansion
* **Description**: Monitors keypress buffer for slash triggers (`/key`). Upon typing `Space` after a valid trigger key, replaces the trigger with resolved value:
  * `/phone` $\rightarrow$ `+1-555-0199`
  * `/date` $\rightarrow$ `August 5, 2026`
  * `/leave` $\rightarrow$ `Dear Manager, Please accept my formal leave application...`

### FEAT-11: Global Keybinding Capture & Live Recorder
* **Description**: System-wide global hotkeys for instant actions.
* **Live Recorder**: Clicking a shortcut row in the Shortcuts tab activates a live `keydown` event listener. Pressing any combination (e.g. `Ctrl+Alt+M`) updates the binding immediately with <kbd>Esc</kbd> cancellation.

### FEAT-12: Dual AI Copilot Failover Architecture
* **Primary**: Groq API (`llama-3.3-70b-versatile`).
* **Failover**: Cerebras API (`llama3.1-8b`).
* **Behavior**: If Groq returns rate limit error HTTP 429 or connection timeout, KeyMind automatically routes the prompt to Cerebras in < 50ms without user interruption.
