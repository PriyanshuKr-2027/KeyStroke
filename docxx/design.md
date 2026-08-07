# KeyStroke — Refined Design System & Interface Specification

## 1. Design Philosophy

KeyStroke's visual architecture is clean, minimalist, and non-intrusive. It uses standard, intuitive UI conventions that respects user focus:
- **No Jargon**: Standard theme labels (**Light**, **Dark**, **System Default**).
- **Clean Surfaces**: No decorative dots, fake sparklines, or non-functional visual noise.
- **Zero Commercialization**: 100% free open-source software (SignPath Foundation code-signed). Zero references to plans, billing, or upgrades.
- **Auto-Configuring Onboarding**: First-run setup collects basic profile fields (First Name, Last Name, Email, Date of Birth) and automatically creates matching text shortcuts (`/name`, `/email`, `/dob`, `/firstname`).

---

## 2. Theme Architecture & Color System

Theme configuration is located inside **Settings $\rightarrow$ General / Appearance** with 3 standard options:
1. **Light**: Warm paper canvas (`#FAF8F5`), crisp white cards (`#FFFFFF`), dark primary text (`#1E1E1E`).
2. **Dark**: Deep charcoal canvas (`#1B1917`), dark cards (`#282522`), light primary text (`#ECE9E3`).
3. **System Default**: Automatically matches OS light/dark preference (`prefers-color-scheme`).

---

## 3. Exhaustive UI Component & Screen Specification

### A. Main Shell & Navigation

#### 1. Left Fixed Sidebar (`Sidebar.tsx`)
* **Width**: Fixed $220\text{px}$.
* **Header**:
  * Product Title: `"KeyStroke"` ($15\text{px}$ semibold).
  * Interceptor Active Toggle Switch:
    * Simple ON / OFF toggle pill controlling background keyboard hook state.
* **Navigation Items**:
  1. **Home** (Icon: `Home`, ID: `dashboard`)
  2. **My Dictionary** (Icon: `Book`, ID: `memory`)
  3. **Text Shortcuts** (Icon: `Code2`, ID: `variables`)
  4. **Grammar & Auto-Fix** (Icon: `CheckCheck`, ID: `grammar`)
  5. **App Rules** (Icon: `AppWindow`, ID: `apps`)
  6. **Keybindings** (Icon: `Keyboard`, ID: `shortcuts`)
* **Bottom Pinned Actions**:
  * **Settings Button** (Icon: `Settings`): Opens Settings Modal.
  * **Setup Wizard Button** (Icon: `HelpCircle`): Re-opens Onboarding Wizard.

#### 2. Top Window Header Bar (`App.tsx`)
* **Height**: $44\text{px}$ draggable region (`data-tauri-drag-region`).
* **Content**: Clean version tag `KeyStroke v0.1.0`. (Theme toggle moved inside Settings).

---

### B. Tab 1: Home Dashboard (`DashboardTab.tsx`)

1. **Header Bar**:
   * Title: `"Dashboard"` ($22\text{px}$ semibold).
   * Subtitle: `"System-wide writing intelligence active"`.

2. **Metric Cards Grid** (Clean, SVG sparklines removed):
   * **Card 1: Words Typed Today**: Displays numeric count of processed words.
   * **Card 2: Typos Auto-Fixed**: Displays count of automatic corrections made.
   * **Card 3: Text Shortcuts Used**: Displays count of variable expansions executed.

3. **Interactive Typing Sandbox**:
   * Label: `"INTERACTIVE TYPING SANDBOX"`.
   * Textarea (`id="sandbox-textarea"`): Allows live testing of autocorrect and next-word prediction.
   * Result Chips: Clean text output showing `Autocorrect: original → corrected` and `Next-word: candidate`.

4. **Recent Automatic Corrections**:
   * Clean table listing recent corrections (`original → corrected`, timestamp).
   * **Undo Button** (`Undo2` icon): Reverts correction.

---

### C. Tab 2: My Dictionary (`MemoryTab.tsx`)

1. **Header Bar**: Title `"My Dictionary"`, `"Add Word"` button, Search input.
2. **Whitelisted Words List**: Word label, Date added, Delete button (`Trash2`).
3. **Add Word Modal**: Input for custom technical terms/jargon, Submit, Cancel.

---

### D. Tab 3: Text Shortcuts (`VariablesTab.tsx`)

1. **Header Bar**: Title `"Text Shortcuts"`, `"Export Pack"` button, `"New Shortcut"` button.
2. **Auto-Generated Profile Shortcuts**:
   * `/name` $\rightarrow$ `First Last`
   * `/email` $\rightarrow$ `user@email.com`
   * `/dob` $\rightarrow$ `YYYY-MM-DD`
   * `/firstname` $\rightarrow$ `First`
3. **Filter & Search Bar**: Filter tabs (`All`, `Static`, `Dynamic`, `AI`), Search input.
4. **Snippets Table**: Displays `/trigger` key, value/template, Edit button, Delete button.
5. **New / Edit Snippet Modal**: Trigger key input (prefix `/`), Type selector, Value input, Save, Cancel.

---

### E. Tab 4: Grammar & Auto-Fix (`GrammarTab.tsx`)

1. **Header Bar**: Title `"Grammar & Correction Engines"`.
2. **Operating Mode Selector**: `Aggressive` (Fix on space/punctuation), `Passive` (Underline only), `Off` (Pause automatic grammar).
3. **Writing Assistant Toggles**:
   * Grammar & Punctuation Assistant (Toggle)
   * Smart Autocorrect (Toggle)
   * Homophone Fixer (Toggle)
   * Predictive Typing Suggestions (Toggle)
4. **Live Grammar Sandbox**: Textarea for testing sentence grammar rules.

---

### F. Tab 5: App Rules (`AppsTab.tsx`)

1. **Header Bar**: Title `"App Rules"`.
2. **Detected Applications List**: App icon/name, Autocorrect Toggle, Grammar Toggle, AI Toggle, Blocked status pill, App Actions menu (`Block app`, `Reset default`).

---

### G. Tab 6: Keybindings (`ShortcutsTab.tsx`)

1. **Header Bar**: Title `"Shortcuts"`, `"Reset Defaults"` button.
2. **Global Shortcuts List**:
   * `Ctrl+Alt+Space`: AI Copilot Palette
   * `Ctrl+Alt+G`: Grammar Fix Selection
   * `Ctrl+Alt+P`: Professional Rewrite Selection
   * `Ctrl+Alt+S`: Summarize Selection
   * `Ctrl+Alt+X`: AI Expand Prompt
   * `Ctrl+Alt+K`: Toggle Interceptor ON/OFF
3. **Recording Modal**: Press key combination, conflict check, Esc to cancel.

---

### H. Settings Modal (`SettingsTab.tsx`)

* **Modal Sections**:
  1. **General**:
     * **Appearance**: Theme selection radio/select (**Light**, **Dark**, **System Default**).
     * Keyboard shortcuts: `"Change"` button $\rightarrow$ Navigates to Keybindings tab.
     * Sound feedback on autocorrect: Toggle.
  2. **System**:
     * Launch at login: Toggle.
     * Minimize to system tray: Toggle.
     * Show in taskbar: Toggle.
  3. **AI & Copilot**:
     * Groq API Key input & status badge.
     * Cerebras API Key input & status badge.
     * Automatic failover toggle.
  4. **Account & Profile**:
     * First Name, Last Name, Email, Date of Birth fields.
     * Save Profile button.
  5. **Data & Privacy**:
     * Export all local data button.
     * Clear activity history button.
     * Purge database button.
