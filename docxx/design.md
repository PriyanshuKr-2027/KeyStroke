# KeyMind Control Center — UI/UX Design Specification
### Reference Visual Language: Wispr Flow (Light, Native, Minimal)

---

## 1. Design Philosophy

KeyMind Control Center follows the same design philosophy as Wispr Flow: a tool that gets out of the way. The UI is a light-mode, native-feeling desktop window — white backgrounds, hairline dividers, generous whitespace, and zero decorative chrome. The app should feel like a system preference panel built by a great product designer, not a dashboard.

Every screen has one job. Navigation is shallow. Controls are exactly where you expect them. The app's intelligence lives in the engine running in the background — the UI just gives you visibility and control over it.

---

## 2. Window Shell & Layout

### Overall Structure
A fixed 2-column layout with a persistent left sidebar and a scrollable content area on the right. No top navigation bar. No tabs across the top. The sidebar is the only navigation.

```
+----------------+----------------------------------------------------------+
|  KeyMind  PRO  |  [Page Title]                         [Primary CTA btn]  |
|                |----------------------------------------------------------|
|  Home          |                                                          |
|  Dictionary    |  [Page Content Area — scrollable]                        |
|  Snippets      |                                                          |
|  Variables     |                                                          |
|  Grammar       |                                                          |
|  App Rules     |                                                          |
|  Shortcuts     |                                                          |
|                |                                                          |
|  ─────────     |                                                          |
|  Settings      |                                                          |
|  Help          |                                                          |
+----------------+----------------------------------------------------------+
```

### Sidebar Dimensions
- **Width**: 200px fixed, not resizable
- **Background**: `#FFFFFF`
- **Right border**: 1px solid `#EBEBEB`
- **Top section**: Logo + plan badge, 24px padding, 60px tall
- **Nav items**: 40px tall, 12px horizontal padding, 4px border-radius on active pill
- **Bottom section**: pinned to bottom, separated from nav by a spacer, contains Settings and Help

### Content Area
- **Background**: `#FFFFFF`
- **Left/right padding**: 48px
- **Top padding**: 40px
- **Max content width**: 760px (centered if window is wider)
- **Page title**: 24px, semibold, `#111111`, flush left
- **Primary CTA button** (where applicable): top-right of content area, dark filled, 14px

---

## 3. Visual Identity

### Color Palette
KeyMind uses a near-monochrome light palette. The only accent colors are functional — they communicate state, not decoration.

| Role | Value | Usage |
|---|---|---|
| **Surface** | `#FFFFFF` | Window background, sidebar, modals |
| **Surface Raised** | `#F5F5F5` | Input fields, settings row groups, list rows |
| **Border** | `#EBEBEB` | Dividers, row separators, sidebar edge |
| **Text Primary** | `#111111` | Page titles, nav labels, row content |
| **Text Secondary** | `#6B6B6B` | Subtitles, descriptions, timestamps, hints |
| **Text Tertiary** | `#AAAAAA` | Placeholders, disabled states, metadata |
| **Callout Background** | `#FAFAE8` | Feature introduction / onboarding cards |
| **Toggle On** | `#22C55E` | Active toggle switches |
| **Toggle Off** | `#D1D5DB` | Inactive toggle switches |
| **Button Fill** | `#111111` | Primary CTA buttons |
| **Button Text** | `#FFFFFF` | Text on filled buttons |
| **Destructive** | `#EF4444` | Delete, block, purge actions |
| **Status Green** | `#22C55E` | Engine running, API connected |
| **Status Amber** | `#F59E0B` | Degraded, slow response, warning |
| **Status Red** | `#EF4444` | Engine stopped, API error |

No glassmorphism. No backdrop blur. No dark theme. No terracotta accent. The interface is white and functional.

### Typography
| Role | Font | Size | Weight |
|---|---|---|---|
| **Page Title** | Inter | 22px | 600 |
| **Callout Headline** | Georgia / serif | 28px | 400 |
| **Nav Label** | Inter | 14px | 400 |
| **Body / Row Label** | Inter | 14px | 400 |
| **Row Subtitle** | Inter | 13px | 400 |
| **Trigger / Code** | JetBrains Mono | 13px | 400 |
| **Button** | Inter | 14px | 500 |
| **Timestamp / Meta** | Inter | 12px | 400 |

Inter is the system-close workhorse for all UI controls. Georgia (or the OS serif fallback) is used exclusively inside callout/feature cards to give them a distinct editorial voice — exactly as Wispr Flow uses a large serif display headline in its callout cards. JetBrains Mono is used for snippet triggers, variable keys, hotkey combinations, and any code-like content.

### Iconography
- Style: Lucide icons (outline, 2px stroke), 18px size
- Color: `#6B6B6B` inactive, `#111111` active/hover
- Sidebar icons: 18px, left-aligned with 8px gap to label
- Inline action icons (edit, delete): 16px, visible on row hover only

---

## 4. Core UI Components

### A. Sidebar Nav Item
```
[ icon ]  Label
```
- Default: text `#6B6B6B`, no background
- Hover: background `#F5F5F5`, text `#111111`, border-radius 6px
- Active: background `#F0F0F0`, text `#111111`, border-radius 6px
- No colored left border stripe. No underline. No bold.

### B. Callout / Feature Card
Used at the top of Dictionary, Snippets, Variables, and Shortcuts pages as a dismissible introduction. Mirrors Wispr Flow's cream feature cards exactly.

```
┌──────────────────────────────────────────────────────────┐  ← border-radius 12px
│                                                    [✕]   │  ← background #FAFAE8
│  The stuff you shouldn't have to retype.                 │  ← Georgia 28px
│                                                          │
│  KeyMind watches for your triggers — type /date and      │  ← Inter 14px #6B6B6B
│  it expands instantly. Add anything you type often.      │
│                                                          │
│  [ /date → August 5, 2026 ]  [ /email → ... ]  [/phone] │  ← pill chips, border
│                                                          │
│  [ Add new variable ]                                    │  ← dark filled button
└──────────────────────────────────────────────────────────┘
```
- Background: `#FAFAE8`
- Headline: Georgia, 28px, `#111111`
- Body: Inter, 14px, `#6B6B6B`
- Example chips: `1px solid #D0D0D0` border, `#FFFFFF` background, Inter 13px, JetBrains Mono for trigger part
- CTA button: `#111111` fill, `#FFFFFF` text, 8px border-radius
- Dismiss X: top-right, `#AAAAAA`, removes card and remembers dismissal

### C. List Row
Standard content row used in Dictionary, Snippets, Variables, App Rules, Shortcuts.

```
  Label or trigger → expansion text                [ ✎ ]  [ 🗑 ]
  ─────────────────────────────────────────────────────────────── ← 1px #EBEBEB
```
- Height: 48px
- Padding: 0 4px
- Hover: background `#FAFAF A`, edit and delete icons appear (opacity 0 → 1)
- Edit icon: pencil, `#AAAAAA`, hover `#111111`
- Delete icon: trash, `#AAAAAA`, hover `#EF4444`
- Trigger portion rendered in JetBrains Mono `#111111`
- Arrow `→` and expansion text in Inter `#6B6B6B`
- No card border wrapping the list — rows sit directly on white

### D. Settings Row Group
Used inside the Settings modal and Grammar tab for grouped toggles and controls.

```
┌──────────────────────────────────────────────────────────┐ ← #F5F5F5 background
│  Launch at login                              [toggle]   │ ← 48px row
│  ─────────────────────────────────────────────────────── │ ← 1px #EBEBEB
│  Minimize to system tray on close             [toggle]   │
│  ─────────────────────────────────────────────────────── │
│  Sound feedback on autocorrect                [toggle]   │
└──────────────────────────────────────────────────────────┘ ← border-radius 10px
```
- Group background: `#F5F5F5`, border-radius 10px
- Row height: 48px, 16px horizontal padding
- Label: Inter 14px `#111111`
- Subtitle (where present): Inter 13px `#6B6B6B`, below label
- Divider: 1px `#EBEBEB`, inset 16px from left
- Toggle: standard pill shape, 44×24px, green `#22C55E` when on
- For rows with a "Change" button instead of a toggle: button is `#F0F0F0` fill, `#111111` text, 8px border-radius

### E. Sub-tab Bar
Used on Dictionary, Snippets, Variables pages to filter between All / Personal / Shared.

```
  All   Personal   Shared with team
  ───
```
- All tabs: Inter 14px `#6B6B6B`
- Active tab: `#111111`, 2px `#111111` underline, flush to bottom of tab bar
- No background highlight on active tab
- Tab bar bottom border: 1px `#EBEBEB` full width
- Sits directly below the page title row, above the callout card

### F. Primary Button
```
[ Add new ]
```
- Background: `#111111`
- Text: `#FFFFFF`, Inter 14px, weight 500
- Padding: 10px 20px
- Border-radius: 8px
- Hover: `#333333`
- Disabled: `#D1D5DB` background, `#9CA3AF` text

### G. Status Pill
Small inline indicator used in the Home tab status bar.

```
● Engine running     ● Grammar server     ● Groq API
```
- Dot: 8px circle, colored by state (green / amber / red)
- Label: Inter 13px `#6B6B6B`
- Arranged horizontally with 24px gaps, no card wrapping them

### H. Toggle Switch
- Size: 44px × 24px
- On: `#22C55E` track, white thumb
- Off: `#D1D5DB` track, white thumb
- Transition: 150ms ease

---

## 5. Page Specifications

### Page 1: Home

**Layout:**
```
Welcome back, [Name]    🔥 5-day streak  |  ✦ 432 words  |  ⚡ 98 WPM
────────────────────────────────────────────────────────────────────────

[ Callout card — only shown first week ]
KeyMind types the way you think.
Works in every app. Type /date, /email, or any trigger and KeyMind
expands it instantly — with grammar fixes happening silently in the background.
[ /date → August 5, 2026 ]  [ teh → the ]  [ there → their ]
[ See how it works ]

────────────────────────────────────────────────────────────────────────

ENGINE STATUS
● Keyboard interceptor running   ● Grammar server connected   ● Groq API ready

────────────────────────────────────────────────────────────────────────

TODAY — AUGUST 5, 2026

  2:41 PM   teh → the   (VS Code)                          [ Undo ]
  2:39 PM   /email → alex@company.com   (Slack)             [ Undo ]
  2:35 PM   there books → their books   (Notion)            [ Undo ]
  2:30 PM   recieve → receive   (Chrome)                    [ Undo ]
```

**Elements:**
- Greeting: Inter 22px semibold `#111111` + inline streak stats (emoji + value + label, `#6B6B6B`)
- Stats bar: streak count, words corrected today, WPM — separated by `|` dividers
- Callout card: `#FAFAE8`, Georgia headline, dismissible, shown for the first 7 days or until dismissed
- Engine status: three status pills in a horizontal row, no card border
- Activity log: date label in `#AAAAAA` caps 12px, then rows with timestamp + correction + app name + Undo button
- Undo button: text-only, `#6B6B6B`, 13px, underline on hover

**Interactive Engine Sandbox** (below activity log):
```
[ Try it: type something here to test KeyMind live... ]
```
- Full-width input, `#F5F5F5` background, 14px placeholder, 12px border-radius
- As user types, grammar and autocorrect fire in real time inside the field
- Corrections show as strikethrough-old → new inline

---

### Page 2: Dictionary

**Header row:** `Dictionary` (title left) + `Add new` button (top right)

**Sub-tabs:** All | Personal | Shared with team

**Callout card** (dismissible):
> *KeyMind learns the way you speak.*
> Add personal terms, technical jargon, client names, or abbreviations. KeyMind will never flag them as typos.
> [ SymSpell ]  [ SQLite ]  [ btw → by the way ]  [ Priyanshu ]
> [ Add new word ]

**List rows** (below callout, after dismiss):
```
  SymSpell                                              [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────
  SQLite                                                [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────
  btw → by the way                                      [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────
  keymind-autocorrect                                   [ ✎ ]  [ 🗑 ]
```
- Plain words: Inter 14px `#111111`
- Abbreviation expansions (`btw → by the way`): trigger in JetBrains Mono, arrow + expansion in `#6B6B6B`
- Header row above list: `PERSONAL DICTIONARY` in Inter 11px `#AAAAAA` caps, 24px top margin

**Empty state:**
```
         📖
   No words added yet.
   Add technical terms, names, or abbreviations
   that KeyMind should never flag.
   [ Add your first word ]
```
Centered, `#6B6B6B` body, dark filled button.

---

### Page 3: Snippets & Variables

**Header row:** `Snippets & Variables` + `Add new` button

**Sub-tabs:** All | Static | Dynamic | AI Prompts

**Callout card** (dismissible):
> *The stuff you shouldn't have to retype.*
> Save shortcuts for everything you type all the time — emails, dates, addresses, templates. Type the trigger and KeyMind expands it instantly.
> [ /email → alex@... ]  [ /date → August 5, 2026 ]  [ /reply → AI draft ]
> [ Add new variable ]

**List rows:**
```
  /email     →   alex@company.com                           [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────────
  /date      →   August 5, 2026   (dynamic)                 [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────────
  /phone     →   +1-555-0199                                [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────────
  /reply     →   Draft polite email reply to clipboard...   [ ✎ ]  [ 🗑 ]
  ──────────────────────────────────────────────────────────────────────
  /leave     →   Dear Manager, Please accept my forma...    [ ✎ ]  [ 🗑 ]
```
- Trigger (`/email`): JetBrains Mono 13px `#111111`
- Arrow `→`: Inter `#AAAAAA`
- Expansion text: Inter 14px `#6B6B6B`, truncated with ellipsis at 400px
- Type badge for dynamic/AI rows: tiny pill `#F0F0F0` border, `#6B6B6B` text, 11px — e.g. `dynamic`, `ai`

**Add / Edit Variable Modal:**
Slides in from the right or opens centered. Contains:
- Trigger key field (JetBrains Mono input)
- Type selector: Static | Dynamic | AI Prompt (segmented control)
- Value or prompt textarea
- Preview output (live resolve for dynamic types)
- Save / Cancel buttons

---

### Page 4: Grammar & Autocorrect

**Header:** `Grammar & Autocorrect` (no CTA button — all controls are inline)

**Section 1 — Operating Mode:**

```
┌─────────────────────────────────────────────────────────────┐
│  Auto-fix mode                                              │
│  ─────────────────────────────────────────────────────────  │
│  Aggressive — fix on Space / punctuation     ( ● )         │
│  ─────────────────────────────────────────────────────────  │
│  Suggestions only — show tooltip, wait for accept  ( ○ )   │
└─────────────────────────────────────────────────────────────┘
```
Radio group styled as a settings row group.

**Section 2 — Engine Controls:**
Settings row group with toggles:
- SymSpell Autocorrect `[toggle]`
- Homophone Resolution `[toggle]`
- LanguageTool Grammar Engine `[toggle]`
- Next-Word Prediction `[toggle]`

**Section 3 — Sensitivity:**
```
  Correction confidence threshold
  ────────────────────────────────────
  [ ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━○ ]  90%
```
Horizontal slider, `#111111` track fill, range 70–99%, label shows live value.

**Section 4 — Language:**
Settings row: `Primary language` → `English (US)` with `Change` button opening a language picker.

**Section 5 — Interactive Sandbox:**
```
Test your grammar engine live:
[ Type something here — corrections appear as you type...  ]
```
Full-width input with live corrections rendering inline, same as Home sandbox.

**Section 6 — Recent Corrections Log:**
Date-grouped list of recent rule triggers with columns: timestamp, original → corrected, rule type (Subject-Verb / Homophone / Typo / Punctuation), app name.

---

### Page 5: App Rules & Exclusions

**Header:** `App Rules`

**Callout card** (shown once, dismissible):
> *KeyMind, everywhere — except where you say.*
> Disable features for specific apps. Block it entirely in password managers or banking apps.
> [ VS Code ]  [ Slack ]  [ 1Password ]  [ Chrome ]
> [ Manage apps ]

**App list** (auto-detected from running/installed apps):
```
  [icon]  VS Code                Autocorrect [on]  Grammar [on]  AI [on]   [ ··· ]
  ──────────────────────────────────────────────────────────────────────────────────
  [icon]  Slack                  Autocorrect [on]  Grammar [on]  AI [on]   [ ··· ]
  ──────────────────────────────────────────────────────────────────────────────────
  [icon]  1Password              [  BLOCKED  ]                              [ ··· ]
  ──────────────────────────────────────────────────────────────────────────────────
  [icon]  Chrome                 Autocorrect [on]  Grammar [off] AI [on]   [ ··· ]
```
- App icon: 24px, sourced from OS
- App name: Inter 14px `#111111`
- Feature toggles: small inline toggles (32×18px), green/grey
- BLOCKED badge: `#FEE2E2` background, `#EF4444` text, 6px border-radius
- `···` menu: opens inline dropdown with "Block app" / "Reset to default"

---

### Page 6: Shortcuts & Hotkeys

**Header:** `Shortcuts`

**Callout card** (dismissible):
> *One key combination. Instant action.*
> Press any shortcut below to trigger KeyMind instantly across any app. Click a row to record a new combination.
> [ Ctrl+Alt+Space ]  [ Ctrl+Alt+G ]  [ Ctrl+Alt+K ]
> [ Customize shortcuts ]

**Shortcut list:**
```
  AI Copilot Palette                         Ctrl + Alt + Space    [ ✎ ]
  ──────────────────────────────────────────────────────────────────────
  Grammar Fix Selection                      Ctrl + Alt + G        [ ✎ ]
  ──────────────────────────────────────────────────────────────────────
  Tone Rewriter — Formal                     Ctrl + Alt + P        [ ✎ ]
  ──────────────────────────────────────────────────────────────────────
  Summarize Selection                        Ctrl + Alt + S        [ ✎ ]
  ──────────────────────────────────────────────────────────────────────
  Toggle Interceptor On/Off                  Ctrl + Alt + K        [ ✎ ]
```
- Action name: Inter 14px `#111111`
- Keybinding: JetBrains Mono 13px `#111111`, right-aligned
- Edit icon: pencil, appears on hover

**Live Recording State** (after clicking edit on a row):
```
  AI Copilot Palette     [ Press any key combination...  Esc to cancel ]
```
- The keybinding cell becomes a highlighted capture zone: `#F5F5F5` background, `1px solid #111111` border, pulsing cursor
- Any key combo pressed immediately populates and saves
- Esc cancels and restores previous binding

---

## 6. Settings Modal

Settings is triggered from the sidebar. It opens as a **centered modal overlay** (not a full navigation tab), identical to Wispr Flow's pattern. The background dims to `rgba(0,0,0,0.3)`.

**Modal dimensions:** 860px wide × 560px tall, border-radius 16px, `#FFFFFF` background, native drop shadow.

**Modal layout:**
```
┌─────────────────────────────────────────────────────────────────────────┐
│  SETTINGS                    │  [Selected section title]                │
│  ──────────────────────      │                                          │
│  General                     │  [Section content]                       │
│  System                      │                                          │
│  AI & Copilot                │                                          │
│                              │                                          │
│  ACCOUNT                     │                                          │
│  Account                     │                                          │
│  Plans & Billing             │                                          │
│  Data & Privacy              │                                          │
│                              │                                          │
│  v1.0.0  [!]                 │                                          │
└─────────────────────────────────────────────────────────────────────────┘
```
- Left panel: 200px, `#FFFFFF`, section headers in Inter 11px `#AAAAAA` caps
- Right panel: fills remainder, 32px padding, scrollable
- Active sub-nav item: `#F0F0F0` background pill, `#111111` text

### Settings — General
Settings row group:
- Keyboard shortcuts → `Change` button (opens shortcut recorder)
- Primary language → `English (US)` → `Change`
- Sound feedback on autocorrect → toggle

### Settings — System
Settings row group — App settings:
- Launch at login → toggle (on by default)
- Minimize to system tray on close → toggle (on by default)
- Show in taskbar → toggle

Settings row group — Sound:
- Autocorrect sound effect → toggle
- Prediction chip sound → toggle

### Settings — AI & Copilot
Settings row group:
- AI provider: Groq (primary) → input field for API key, inline latency test pill
- Failover provider: Cerebras → input field for API key, inline latency test pill
- Failover behavior: Auto (on 429/5xx) → toggle, always shown enabled

API key fields: `#F5F5F5` background, JetBrains Mono 13px, show/hide toggle, "Test connection" text button inline.

Latency pill (after successful test): `P99 < 48ms` in `#DCFCE7` background, `#16A34A` text.

### Settings — Account
- First name / Last name: editable text inputs
- Email: read-only, `#6B6B6B`
- Profile picture: avatar circle with initials, "Change photo" link
- Actions: `Sign out` (outline button) + `Delete account` (destructive, red text link) + `Save` (primary filled, right-aligned)

### Settings — Plans & Billing
- Current plan badge, usage meters, upgrade CTA.

### Settings — Data & Privacy
- 100% local processing confirmation row (read-only, green status dot)
- Export all data → button
- Clear activity history → button (`#EF4444` text)
- Purge database → destructive button with confirmation dialog

---

## 7. Onboarding Wizard (First Run)

Shown full-screen on first launch, replacing the normal shell. Three steps with a progress indicator.

**Progress bar:** 3 dots at top center, filled dot for current step, empty for upcoming.

### Step 1 — Permissions
```
        KeyMind needs one permission to get started.

   To type intelligently across all your apps, KeyMind needs
   low-level keyboard access. This runs entirely on your device —
   nothing leaves your machine.

        [ Open Accessibility Settings ]  [ Continue → ]

        Status: ● Waiting for permission
                ● Granted  ← turns green when detected
```
- Large centered layout, max-width 480px
- Headline: Inter 22px semibold
- Body: Inter 15px `#6B6B6B`
- **Interactive Triggers**: Clicking `[ Open Accessibility Settings ]` dispatches the native `open_accessibility_settings` Tauri IPC command to open OS system preferences directly.
- **Auto-Advance & Continue**: Status indicator polls permission state every 500ms and automatically advances to Step 2 upon detection, with an explicit `Continue →` button provided for manual progression.


### Step 2 — AI Setup
```
        Connect your AI keys (optional).

   KeyMind uses Groq and Cerebras for AI-powered features — 
   tone rewrites, /reply expansions, and the AI Copilot palette.
   You can skip this and add them later in Settings.

   Groq API key        [ ________________________ ]  [ Test ]
   Cerebras API key    [ ________________________ ]  [ Test ]

        [ Skip for now ]    [ Continue → ]
```

### Step 3 — Preset
```
        How do you type?

   ( )  Power Developer
        Fast triggers, aggressive autocorrect, technical whitelist on.

   ( )  Business & Executive
        Formal grammar, email templates, polite AI rewrites.

   ( )  Minimalist
        Next-word prediction only. Non-intrusive, no auto-changes.

                              [ Finish setup → ]
```
Radio cards: `#F5F5F5` background, `1px solid #EBEBEB` border, border becomes `1px solid #111111` when selected. 12px border-radius.

---


---

## 8. AI Copilot Palette (Ctrl+Space)

### Overview

The AI Copilot Palette is a floating prompt bar that opens system-wide on `Ctrl+Space`. It is the fastest path to AI assistance — one shortcut, one input, one result. The user types anything in plain language and KeyMind handles the rest.

The palette has a single mode. There are no tabs, no action chips, no preset buttons. Just a prompt input. The intelligence is in the engine, not the UI.

### Trigger & Positioning

- **Shortcut**: `Ctrl+Space` (system-wide, registered via Win32 `RegisterHotKey`)
- **Position**: Horizontally centered on the primary monitor, vertically at 38% from the top — slightly above center, where the eye naturally rests
- **Size**: 560px wide, height grows with content (min ~90px, max ~260px)
- **z-order**: Always on top (`HWND_TOPMOST`) while open, relinquished on close

### Context Capture

When `Ctrl+Space` fires, KeyMind silently reads the text surrounding the cursor in the active text field before the palette opens. This happens in < 30ms via the Win32 accessibility API (`IUIAutomation`). The captured context (up to ~500 characters centered on the cursor position) is displayed in a muted strip at the top of the palette so the user can verify what the model will see.

If no text field is focused at the moment of the shortcut, the context strip shows a muted message: "No text field focused — result will be copied to clipboard."

### Layout (all states share this shell)

```
┌────────────────────────────────────────────────────────────┐
│  context  …grabbed text shown here, 2 lines max, muted…    │  ← context strip
├────────────────────────────────────────────────────────────┤
│  [★]  Type anything…                          ↵ enter      │  ← input row
├────────────────────────────────────────────────────────────┤
│  ● Result types into [App Name]           esc to close     │  ← footer
└────────────────────────────────────────────────────────────┘
```

### Component Specs

**Palette shell**
- Background: `#FFFFFF` (light) / `var(--surface-2)` 
- Border: `0.5px solid var(--border-strong)`
- Border-radius: 14px
- Shadow: `0 8px 32px rgba(0,0,0,0.10), 0 2px 8px rgba(0,0,0,0.06)`

**Context strip** (top section)
- Background: `var(--surface-1)` — slightly off-white to distinguish from input area
- Border-bottom: `0.5px solid var(--border)`
- Padding: `10px 14px 8px`
- Label: `context` in JetBrains Mono 11px `var(--text-muted)`, uppercase tracking
- Text: Inter 12px `var(--text-secondary)`, max 2 lines, overflow ellipsis
- Highlighted portion (the most relevant grabbed sentence): Inter 12px `var(--text-primary)` weight 500
- When no text field active: background `var(--surface-0)`, text `var(--text-muted)` italic

**Input row** (middle section)
- Padding: `12px 14px`
- KeyMind icon: 22×22px rounded square, `var(--fill-primary)` background, white star/spark SVG
- Input field: Inter 14px, `var(--text-primary)`, no border, transparent background, full flex width
- Placeholder: `var(--text-muted)` — "Ask anything — rewrite, explain, continue, translate…"
- Enter hint: JetBrains Mono 11px, `var(--surface-1)` background, `0.5px var(--border)`, border-radius 5px, text `var(--text-muted)`

**Footer bar** (bottom section)
- Border-top: `0.5px solid var(--border)`
- Padding: `6px 14px 8px`
- Left: output destination indicator
  - Green dot (6px circle, `var(--fill-success)`) + "Result types into [App Name]" in Inter 11px `var(--text-muted)`
  - Grey dot when no text field: "Result copied to clipboard"
  - App name is read from the active window title at trigger time
- Right: "esc to close" in JetBrains Mono 11px `var(--text-muted)`

### States

**1. Idle (text field active)**
Context strip shows grabbed text. Input focused, cursor blinking. Footer shows green dot + target app name. User types prompt and hits Enter.

**2. Idle (no text field)**
Context strip shows muted "no text field" message with `var(--surface-0)` background. Footer shows grey dot + "Result copied to clipboard".

**3. Loading**
Input row replaced by loading row:
- 16px spinner (1.5px border, top segment `var(--text-primary)`, 700ms rotation)
- User's submitted prompt text in Inter 13px `var(--text-secondary)`
- Model label right-aligned: `groq · llama-3.3-70b` in JetBrains Mono 11px `var(--text-muted)`
- Footer: "esc to cancel"

**4. Result**
Input row replaced by result area:
- Result text: Inter 14px `var(--text-primary)`, line-height 1.6, padding `12px 14px 8px`
- Action row below text:
  - **Insert** button (primary filled, `var(--fill-primary)` bg, white text, insert icon) — types result into the text field
  - **Copy** button (outline) — copies to clipboard
  - **Retry** button (outline) — re-runs the same prompt
- Footer: "Ready to insert into [App Name]"
- If no text field was active: Insert button is absent, Copy is primary

**5. Error**
Input row replaced by error row:
- Red dot (6px, `var(--fill-danger)`)
- Error message: Inter 13px `var(--text-danger)` — e.g. "Groq rate limited — switching to Cerebras, retrying…"
- Retry button: outline, `var(--border-danger)` border, `var(--text-danger)` text
- Automatic retry via Cerebras failover happens without user action; this state is shown only if both providers fail

### Behavior & Interaction

| Event | Behavior |
|---|---|
| `Ctrl+Space` | Open palette, capture context, focus input |
| `Esc` (idle) | Close palette, return focus to previous window |
| `Esc` (loading) | Cancel in-flight request, return to idle |
| `Esc` (result) | Close palette |
| `Enter` (idle, empty) | No-op |
| `Enter` (idle, has text) | Submit prompt, enter loading state |
| Click Insert | Type result at cursor in target app, close palette |
| Click Copy | Write result to clipboard, show brief "Copied" flash on button, close palette |
| Click outside palette | Close palette |
| Win32 focus lost | Close palette |

### Animation

| Transition | Spec |
|---|---|
| Palette open | 120ms ease-out, scale 0.96 → 1.0, opacity 0 → 1 |
| Palette close | 100ms ease-in, opacity 1 → 0 |
| Idle → Loading | Input row fades out 80ms, loading row fades in 80ms |
| Loading → Result | Loading row fades out 80ms, result fades in 120ms |
| Error state | Fade-in 100ms |

`prefers-reduced-motion`: all transitions instant (0ms).


---

## 9. Interaction & Motion

Interactions are fast and functional. No animation for animation's sake.

| Interaction | Behavior |
|---|---|
| Sidebar nav click | Instant content swap, no slide/fade |
| Modal open (Settings) | 150ms fade-in + scale from 0.97 → 1.0 |
| Callout card dismiss | 200ms fade-out + collapse height |
| Row hover | 80ms background fade to `#FAFAFA` |
| Toggle switch | 150ms ease thumb slide |
| Gboard chip appear | 100ms fade-in |
| Gboard chip dismiss | 80ms fade-out |
| Live recording capture zone | Pulse border animation (1s loop) while listening |
| Status pill change | 300ms color crossfade |

`prefers-reduced-motion`: all transitions drop to 0ms.

---

## 10. Empty States

Every list page has an empty state for when no entries exist yet.

| Page | Empty State Message | CTA |
|---|---|---|
| Dictionary | "No words added yet. Add technical terms KeyMind should never flag." | Add your first word |
| Snippets & Variables | "No snippets yet. Save the things you type most often." | Add your first snippet |
| App Rules | "No apps detected yet. Open an app and KeyMind will find it." | Refresh |
| Shortcuts | Never empty — defaults always present | — |
| Home activity feed | "Nothing corrected yet today. Start typing in any app." | — |

Empty state layout: centered in content area, icon (Lucide, 32px, `#D1D5DB`), heading Inter 16px `#111111`, body Inter 14px `#6B6B6B`, CTA button below.

---

## 11. Error & Degraded States

| State | Visual Treatment |
|---|---|
| Grammar server offline | Status pill turns amber: `● Grammar server offline` — engine continues with autocorrect only |
| Groq API rate limit | Status pill amber: `● Groq throttled — using Cerebras fallback` |
| Both AI providers down | Status pill red: `● AI Copilot unavailable` — all other features continue |
| LanguageTool timeout | Silent retry × 3, then amber pill. No user interruption. |
| Keyboard interceptor stopped | Full-width banner below status bar: red `● Engine stopped — click to restart` |

Errors never interrupt typing. The daemon is always the priority. The UI reflects state but never blocks.

---

## 12. Window Behavior

- **Minimum window size**: 900px × 600px
- **Default window size**: 1100px × 700px
- **Sidebar**: fixed, not collapsible, not resizable
- **On close**: minimizes to system tray (configurable in Settings)
- **On tray icon click**: restores window to last position and size

---

## 13. Distribution & Installation Packaging

KeyMind provides two official release distribution packages:

1. **Installer Edition**: `KeyStroke_Installer_v0.1.0.exe` (7.5 MB)
   - NSIS setup with `downloadBootstrapper` configuration for automated WebView2 runtime installation.
   - Registers system startup shortcuts and tray service daemon.
2. **Portable Edition**: `KeyStroke_v0.1.0_Portable_x64.zip` (10.8 MB)
   - Zero-installation portable package containing pre-compiled binaries, local SQLite dictionaries, and frontend webview static assets.

- **Window title**: `KeyMind` (no page name in title bar)
