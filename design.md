# KeyMind Control Center — User Product & UI/UX Specification

## 1. Product Vision & User Experience

KeyMind Control Center is a premium, desktop-native productivity suite that enhances system-wide typing across all Windows and macOS applications. Operating quietly in the background via low-level keyboard interception, KeyMind provides sub-1ms next-word prediction, intelligent autocorrect, LanguageTool grammar enhancement, custom snippet variables, and local AI copilot assistance.

---

## 2. End-User Architecture & Navigation Structure

KeyMind is organized into a clean 2-column desktop sidebar window, backed by a system tray icon and floating overlay widgets.

```
+-----------------------------------------------------------------------------------+
|  KeyMind PRO              |  [Tab Header Title]                  [Status Pill]    |
|                           |-------------------------------------------------------|
|  MAIN NAVIGATION          |                                                       |
|  - Dashboard / Home       |  [Active Tab Main Viewport Content Area]              |
|  - Snippets & Variables   |                                                       |
|  - Grammar & Autocorrect  |                                                       |
|  - Memory & Dictionary    |                                                       |
|  - App Rules & Exclusions |                                                       |
|  - Shortcuts & Hotkeys    |                                                       |
|                           |                                                       |
|  SYSTEM                   |                                                       |
|  - Settings & Preferences |                                                       |
|                           |-------------------------------------------------------|
|  [Engine Running Switch]  |                                [Gboard Suggestion Pill] |
+-----------------------------------------------------------------------------------+
```

---

## 3. Onboarding & First-Run Experience

When launched for the first time, KeyMind guides the user through a 3-step interactive onboarding wizard:

### Step 1: System Permissions & Accessibility Grant
* Explains the need for low-level keyboard interception permissions (`SetWindowsHookExW` on Windows, Accessibility API on macOS).
* Provides a one-click button to open System Settings directly.
* Displays a live status indicator that turns green as soon as permission is granted.

### Step 2: Local AI & Copilot Setup
* Configures local AI acceleration keys (Groq API, Cerebras API) or enables 100% Offline Mode.
* Validates key connection in real-time with latency test pills (`P99 < 50ms`).

### Step 3: Preset Configuration
* User selects their typing profile:
  * **Power Developer**: Fast triggers, aggressive autocorrect, technical jargon whitelist enabled.
  * **Business & Executive**: Formal grammar rules, email templates, polite AI rewrites.
  * **Minimalist**: Next-word prediction only, non-intrusive suggestion tooltips.

---

## 4. Complete Page & Tab Specifications

### Tab 1: Dashboard / Home
* **Master Engine Switch**: Large high-contrast toggle to pause or resume keyboard interception instantly.
* **Status Bar**: Real-time health indicators for Keyboard Interceptor, AI Copilot API, and LanguageTool Grammar Server.
* **Daily Impact Metrics**:
  * Words Typed Today (with daily trend indicator).
  * Autocorrects Applied.
  * Snippets & Variables Expanded.
  * Time Saved Counter (calculated based on typing speed).
* **Live Activity Feed**: Scrollable list of recent autocorrects and grammar fixes with one-click "Undo Fix" trigger.
* **Interactive Engine Sandbox**: Inline test input bar allowing users to test typing behavior live inside the app before returning to external software.

### Tab 2: Snippets & Variables Manager
* **Trigger Table**: Searchable list of all text expansions (`/email`, `/date`, `/address`, `/meeting`).
* **Variable Types**:
  * **Static Text**: Instant drop-in expansion (e.g. `/email` $\rightarrow$ `name@domain.com`).
  * **Dynamic Computed**: Auto-calculated values (e.g. `/date` $\rightarrow$ `August 5, 2026`, `/time`, `/clipboard`).
  * **AI Prompt Expansion**: Custom AI prompts (e.g. `/reply` $\rightarrow$ "Draft polite email reply to clipboard content").
* **Actions**: Add New Variable modal, Test Expansion resolution preview, Export/Import JSON backup.

### Tab 3: Grammar & Autocorrect Controls
* **Operating Mode Selection**:
  * **Aggressive Auto-Fix**: Automatically replaces typos and grammar errors upon typing space or punctuation.
  * **Suggestions Only**: Displays a subtle floating tooltip near the cursor without altering text until accepted.
* **Sensitivity Sliders**: Fine-tune confidence thresholds (e.g., 90%–99%) for SymSpell typo correction.
* **Language Selection**: Primary typing language (English US/UK, Spanish, German, French) with automatic dialect handling.
* **Recent Corrections Log**: Detailed audit log of recent rule triggers (Subject-Verb agreement, Homophone confusion, Typo transpositions).

### Tab 4: Memory & Personal Dictionary
* **Personal Dictionary Whitelist**: User-managed list of custom jargon, acronyms, company names, and technical terms to ignore during autocorrect.
* **Auto-Learned Frequent Phrases**: Multi-word phrases automatically learned from the user's typing history, ranked by frequency.
* **Controls**: Pin favorite phrases, unlearn unwanted entries, bulk import dictionary files.

### Tab 5: Per-App Rules & Exclusions
* **Application List**: Automatically detects installed software (e.g., VS Code, Slack, Chrome, Word, Terminal).
* **Granular Toggles**:
  * Enable/disable Autocorrect per application.
  * Enable/disable Grammar checking per application.
  * Enable/disable AI Copilot per application.
* **Blacklist Mode**: One-click "Block App" button to completely disable KeyMind in security-sensitive software (e.g., password managers, financial portals).

### Tab 6: Shortcuts & Hotkeys
* **Conflict-Free Shortcut Registry**:
  * **AI Copilot Palette**: `Ctrl+Alt+Space`
  * **Grammar Fix Selection**: `Ctrl+Alt+G`
  * **Tone Rewriter (Formal)**: `Ctrl+Alt+P`
  * **Summarize Selection**: `Ctrl+Alt+S`
  * **AI Prompt Trigger**: `Ctrl+Alt+X`
  * **Toggle Interceptor**: `Ctrl+Alt+K`
* **Live Recording**: Clicking any row listens for physical key combinations and updates the hotkey in real time.

### Tab 7: Settings & System Preferences
* **General Preferences**:
  * Launch at System Startup toggle.
  * Minimize to System Tray / Menu Bar on Close toggle.
  * Sound Feedback on Autocorrect (Optional subtle click sound).
* **AI API Credentials**: Secure encrypted storage for Groq, Cerebras, or custom OpenAI-compatible endpoints.
* **Data & Privacy**: 100% local telemetry toggle, Database export, Purge history button.

---

## 5. System Tray & Overlay UX Specifications

### A. System Tray / Menu Bar Icon
* **Right-Click Context Menu**:
  * Pause Interceptor / Resume Interceptor
  * Quick Toggle: Aggressive vs. Suggestion Mode
  * Open Control Center...
  * Quit KeyMind

### B. Floating Gboard Next-Word Suggestion Chip
* Displays near the active text cursor when high-confidence predictions are available.
* Displays predicted word, confidence score, and `<kbd>Tab ↹</kbd>` accept key.
* Pressing <kbd>Tab ↹</kbd> inserts the word with trailing space; pressing <kbd-[#71707C]>Esc</kbd> dismisses it.

### C. Floating AI Copilot Palette (`Ctrl+Alt+Space`)
* Compact floating search bar overlay centered on the primary monitor.
* Allows quick AI prompt execution over selected text or clipboard content.

---

## 6. Color System & Aesthetics (Claude Code & Whisper Flow Palette)

* **Background**: Warm Obsidian Slate (`#121215`).
* **Glass Panels**: Warm dark translucent surfaces (`rgba(25, 24, 30, 0.75)` with `backdrop-blur-xl`).
* **Primary Accent**: Claude Terracotta (`#DA7756` / `#C86544`).
* **Secondary Accents**: Warm Amber (`#F59E0B`), Emerald (`#10B981`), Sky (`#0EA5E9`).
* **Typography**: **Plus Jakarta Sans** for UI controls + **JetBrains Mono** for triggers and keybindings.
