# KeyStroke System Architecture & Design Report

This report provides a detailed analysis of the current architecture and design of the **KeyStroke (KeyMind)** system. The system is a high-performance, system-wide AI desktop typing intelligence suite.

## 1. High-Level Architecture Overview

KeyStroke employs a split-architecture model, utilizing a highly optimized, low-level backend combined with a lightweight frontend interface.

*   **Backend (Core Engine & Interceptors)**: Built in **Rust** for memory safety, ultra-low latency, and low resource footprint.
*   **Frontend (Control Center & Palette)**: Built with **Tauri, React, and TypeScript**, providing a cross-platform desktop UI using web technologies.
*   **Communication**: Communication between the UI and the Core Daemon happens via Tauri Commands and JSON-RPC over IPC.

### Architectural Diagram

```mermaid
flowchart TD
    subgraph OS Layer
        HookWin[Windows WH_KEYBOARD_LL]
        HookMac[macOS CGEventTap]
    end

    subgraph Backend Core (Rust)
        Interceptor[keymind-interceptor]
        Engine[keymind-engine\nDaemon]
        Autocorrect[keymind-autocorrect\nSymSpell + Spatial]
        Prediction[keymind-prediction\nONNX + Trigram]
        Grammar[keymind-grammar\nLanguageTool]
        Variables[keymind-variables\nText Expansions]
        DB[(SQLite\ndictionary.db)]
    end

    subgraph External APIs
        Groq[Groq AI]
        Cerebras[Cerebras AI Failover]
        LangTool[LanguageTool Server]
    end

    subgraph Frontend (Tauri/React)
        ControlCenter[Control Center UI]
        Palette[AI Copilot Palette]
    end

    HookWin & HookMac -->|Raw VK Events| Interceptor
    Interceptor -->|WordCompleted Events| Engine
    Engine <--> Autocorrect
    Engine <--> Prediction
    Engine <--> Grammar
    Engine <--> Variables
    Engine <--> DB
    
    Engine <-->|IPC| ControlCenter
    Engine <-->|IPC| Palette
    
    Palette <--> Groq
    Palette <--> Cerebras
    Grammar <--> LangTool
```

---

## 2. Component Deep Dive

The system is modularized into several workspace crates, each responsible for a distinct part of the pipeline.

### 2.1 Low-Level Keyboard Interceptors
*   **Crates**: `keymind-interceptor-windows`, `keymind-interceptor-macos`.
*   **Mechanism**: Uses unbuffered hardware keyboard hooks. In Windows, it operates via a dedicated Win32 message pump thread (`GetMessageW`).
*   **State Management**: 
    *   `INTERCEPTOR_ACTIVE`: Controls pausing/resuming the hook.
    *   `IS_INJECTING`: Guards against processing synthetic key events injected by the app itself (e.g., when sending backspaces for autocorrect).
*   **Buffer**: Characters are buffered into `WORD_BUFFER` between whitespace delimiters, then emitted to the engine.

### 2.2 Text Processing Engines
| Engine | Description | Implementation Details |
| :--- | :--- | :--- |
| **Autocorrect** | Real-time typo fixing | 3 Layers: 10k whitelist bypass $\rightarrow$ Length check $\rightarrow$ SymSpell 82k dictionary lookup. Incorporates a spatial QWERTY matrix to adjust scoring based on physical key adjacency. |
| **Prediction** | Next-word suggestion | Trigram n-gram probabilities powered by an ONNX runtime. Displays via a floating chip. |
| **Grammar** | Full sentence correction | Multi-pass right-to-left offset fixer connected to LanguageTool to prevent character index shifts during replacements. |
| **Variables/Shortcuts** | Snippet expansion | Triggers on `/`. Supports static text and dynamic placeholders like `{date}`, `{time}`, and `{clipboard}`. |

### 2.3 Rejection Memory & Local Storage
*   **Rejection Memory**: If a user rejects an autocorrect (e.g., backspaces over it), the word is added to `user_correction_overrides` (in memory) and skips future autocorrects.
*   **Database**: All learned phrases, whitelists, and text shortcuts are stored locally in a SQLite database (`dictionary.db`) managed via `sqlx`.

### 2.4 AI Copilot Integration
*   The floating AI Palette utilizes a dual-provider strategy. 
*   **Primary**: Groq (`llama-3.3-70b-versatile`) for high-quality, fast responses.
*   **Failover**: Cerebras (`llama3.1-8b`) automatically kicks in if Groq experiences downtime.

---

## 3. UI/UX Design System

The frontend application (`keymind-control-center`, `keymind-palette`) enforces a strictly minimalist, "anti-slop", and non-commercial design language.

### Theme Architecture
The UI is styled around a sophisticated **Claude Theme** with three modes: Light, Dark, and System Default.
*   **Light Theme**: Canvas (`#FAF8F5`), Cards (`#FFFFFF`), Text (`#1E1E1E`), Accents (`#DA7756`).
*   **Dark Theme**: Canvas (`#1B1917`), Cards (`#282522`), Text (`#ECE9E3`), Accents (`#DA7756`).

### Interface Structure
The Control Center utilizes a fixed $220\text{px}$ sidebar for navigation with the following primary tabs:
1.  **Dashboard**: Metrics (Words typed, Typos fixed, Shortcuts used) and an interactive typing sandbox.
2.  **My Dictionary**: Management of whitelisted jargon and custom terms.
3.  **Text Shortcuts**: Interface to create and manage `/` trigger expansions.
4.  **Grammar & Auto-Fix**: Toggles for aggressive/passive modes and individual assistant features.
5.  **App Rules**: Per-application configurations to disable or modify engine behavior in specific software.
6.  **Keybindings**: Global shortcut recording (e.g., `Ctrl+Alt+Space` for Palette).
7.  **Settings**: Theme selection, auto-start, AI API keys, and data management.

---

## 4. Performance & Optimization

Given that KeyStroke runs as a persistent background daemon, resource optimization is heavily engineered.

> [!TIP]
> **Sub-60MB Memory Footprint Strategy**
> The system achieves an ultra-low memory footprint through aggressive tuning:
> 1. **WebView2 Flags**: Passes `--disable-gpu`, `--renderer-process-limit=1`, and `--js-flags="--max-old-space-size=48"` to the Tauri runtime.
> 2. **Win32 Heap Trimming**: A background thread invokes `SetProcessWorkingSetSize` every 30 seconds, manually returning unallocated heap memory to the Windows OS, saving 120MB+ of RAM.

## 5. Summary

The KeyStroke architecture demonstrates an excellent balance between low-level performance (Rust hardware hooks) and maintainable UI (Tauri/React). The modular crate structure allows independent scaling of features like autocorrect, grammar, and AI. The design system is notably refined, focusing on typography, whitespace, and a high-end color palette while avoiding unnecessary visual clutter.
