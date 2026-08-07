# KeyStroke — Complete Features & Capabilities Guide

KeyStroke is a system-wide AI typing intelligence suite designed for frictionless, zero-latency text enhancement across all desktop applications.

---

## 1. Smart Autocorrect Engine

### Zero False Positive Multi-Layered Pipeline
* **Layer 0 (Google-10k Whitelist)**: Contains 4,758 of the most frequent English words. Words like `issue`, `grammar`, `this`, `from`, `with`, `there` are protected by an instant $O(1)$ hash set check and are **never** wrongly corrected.
* **Layer 1 (Short Word Protection)**: Words $\le 3$ characters (e.g. `is`, `in`, `at`, `the`, `ok`) pass through untouched.
* **Layer 2 (SymSpell Distance 1 Gate)**: Edit distance is strictly capped at 1. Sub-millisecond lookup against an 82,000-word frequency dictionary.
* **QWERTY Spatial Error Model**:
  * **Adjacent Key Typos (Fat-Finger)**: Requires $2.5\times$ candidate frequency threshold.
  * **Non-Adjacent Key Typos**: Requires $10.0\times$ candidate frequency threshold.

### Session Rejection Memory
If KeyStroke auto-corrects a word (e.g. `teh` $\rightarrow$ `the`) and you backspace to retype your original word, KeyStroke remembers your choice. The original word is added to a session override memory, ensuring KeyStroke will **never auto-correct that word again** during your session.

---

## 2. Text Shortcuts & Variables (`/` Prefix)

* **Slash Command Triggers**: Variables are triggered strictly by typing `/` followed by the variable key (e.g. `/email`, `/date`, `/address`, `/zoom`).
* **Multi-Placeholder Expansion**:
  * `{date}`: Expands to current date (e.g. `August 07, 2026`).
  * `{time}`: Expands to current local time (e.g. `10:45:00`).
  * `{clipboard}`: Pastes current system clipboard contents directly.
* **Automatic Buffer Reset**: Executes `clear_word_buffer()` immediately after expansion so typing resumes cleanly.

---

## 3. Global AI Shortcuts & Automated Actions

System-wide global hotkeys that work across any desktop text editor:

| Hotkey | Action | Description |
| :--- | :--- | :--- |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Space</kbd> | **Copilot Palette** | Opens floating AI prompt window near your active cursor |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>G</kbd> | **Grammar Fix** | Copies highlighted text, fixes grammar errors, and pastes result back |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>P</kbd> | **Professional Rewrite** | Copies selection, rewrites in formal business tone via AI, and pastes back |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>S</kbd> | **Summarize** | Copies selection, summarizes key points via AI, and pastes back |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>X</kbd> | **AI Expand** | Copies selection, expands prompt with details via AI, and pastes back |
| <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>K</kbd> | **Toggle Interceptor** | Pauses or resumes global keyboard interception system-wide |

---

## 4. Real-Time Interceptor Pause Control

* **Sidebar ON/OFF Toggle Switch**: Header badge in sidebar allows 1-click pause/resume of the background keyboard hook.
* **Zero Overhead when OFF**: When paused, the low-level hook (`WH_KEYBOARD_LL`) returns immediately without recording keystrokes or processing buffers.

---

## 5. Claude Light & Dark Design System

* **Claude Light Theme**: Natural paper tint (`#FAF8F5`), clean white card containers (`#FFFFFF`), warm subtle borders (`#E8E4DC`), dark primary text (`#1E1E1E`), terracotta branding accent (`#DA7756`).
* **Claude Dark Theme**: Deep charcoal tint (`#1B1917`), dark card containers (`#22201D`), dark borders (`#383430`), light primary text (`#ECE9E3`), terracotta branding accent (`#DA7756`).
* **Instant Toggle**: Toggle between light and dark themes from the top window drag region or sidebar button.

---

## 6. Windows Enterprise Deployment & UIPI Bypass

* **Elevated Window Functionality (UIPI Bypass)**: Built with `keystroke.manifest` featuring `uiAccess="true"` and installed in `C:\Program Files\KeyStroke\`. Works seamlessly inside Administrator Command Prompts, Task Manager, and elevated IDEs.
* **SignPath Code Signing**: Integrated with SignPath Foundation OV certificate signing in GitHub Actions CI/CD workflow (`release.yml`).
