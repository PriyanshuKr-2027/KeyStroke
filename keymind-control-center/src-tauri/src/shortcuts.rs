use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub id: String,
    pub label: String,
    pub default_binding: String,
    pub current_binding: String,
}

pub struct ShortcutManager {
    shortcuts: Mutex<Vec<ShortcutConfig>>,
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutManager {
    pub fn new() -> Self {
        let defaults = vec![
            ShortcutConfig {
                id: "copilot_palette".to_string(),
                label: "AI Copilot Palette".to_string(),
                default_binding: "Ctrl+Alt+Space".to_string(),
                current_binding: "Ctrl+Alt+Space".to_string(),
            },
            ShortcutConfig {
                id: "grammar_fix".to_string(),
                label: "Grammar Fix Selection".to_string(),
                default_binding: "Ctrl+Alt+G".to_string(),
                current_binding: "Ctrl+Alt+G".to_string(),
            },
            ShortcutConfig {
                id: "copilot_professional".to_string(),
                label: "Copilot Professional".to_string(),
                default_binding: "Ctrl+Alt+P".to_string(),
                current_binding: "Ctrl+Alt+P".to_string(),
            },
            ShortcutConfig {
                id: "copilot_summarize".to_string(),
                label: "Copilot Summarize".to_string(),
                default_binding: "Ctrl+Alt+S".to_string(),
                current_binding: "Ctrl+Alt+S".to_string(),
            },
            ShortcutConfig {
                id: "ai_expand".to_string(),
                label: "Trigger AI Prompt".to_string(),
                default_binding: "Ctrl+Alt+X".to_string(),
                current_binding: "Ctrl+Alt+X".to_string(),
            },
            ShortcutConfig {
                id: "toggle_engine".to_string(),
                label: "Toggle KeyMind Interceptor".to_string(),
                default_binding: "Ctrl+Alt+K".to_string(),
                current_binding: "Ctrl+Alt+K".to_string(),
            },
        ];

        Self {
            shortcuts: Mutex::new(defaults),
        }
    }
}

#[tauri::command]
pub fn register_global_shortcuts() -> Result<(), String> {
    info!("Global shortcuts registered: Ctrl+Alt+Space, Ctrl+Alt+G, Ctrl+Alt+P, Ctrl+Alt+S, Ctrl+Alt+X, Ctrl+Alt+K");
    Ok(())
}

#[tauri::command]
pub fn get_shortcuts_list(state: tauri::State<ShortcutManager>) -> Vec<ShortcutConfig> {
    state.shortcuts.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

pub fn parse_shortcut_str(s: &str) -> (u32, u32) {
    let mut mods: u32 = 0;
    let mut vk: u32 = 0;

    for part in s.split(&['+', ' '][..]) {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        match p.to_lowercase().as_str() {
            "ctrl" | "control" => mods |= 0x0002,
            "alt" | "option" => mods |= 0x0001,
            "shift" => mods |= 0x0004,
            "win" | "cmd" | "command" | "meta" => mods |= 0x0008,
            "space" => vk = 0x20,
            other => {
                if let Some(ch) = other.chars().next() {
                    if ch.is_ascii_alphanumeric() {
                        vk = ch.to_ascii_uppercase() as u32;
                    }
                }
            }
        }
    }

    (mods, vk)
}

#[tauri::command]
pub fn update_shortcut_binding(
    id: String,
    binding: String,
    state: tauri::State<ShortcutManager>,
) -> Result<(), String> {
    let mut list = state.shortcuts.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(sc) = list.iter_mut().find(|item| item.id == id) {
        sc.current_binding = binding.clone();
        info!("Updated shortcut '{}' to '{}'", id, binding);

        let (mods, vk) = parse_shortcut_str(&binding);
        let numeric_id = match id.as_str() {
            "copilot_palette" => 1,
            "grammar_fix" => 2,
            "copilot_professional" => 3,
            "copilot_summarize" => 4,
            "ai_expand" => 5,
            "toggle_engine" => 6,
            _ => 99,
        };
        keymind_interceptor_windows::lifecycle::update_registered_hotkey(
            numeric_id,
            mods,
            vk,
        );
    }
    Ok(())
}

#[tauri::command]
pub async fn handle_shortcut_trigger(id: String) -> Result<String, String> {
    // 1. Simulate copy
    simulate_copy();

    tokio::time::sleep(std::time::Duration::from_millis(75)).await;

    // 2. Read clipboard
    if let Ok(mut cb) = Clipboard::new() {
        if let Ok(selection) = cb.get_text() {
            info!("Shortcut triggered: {} for selection len {}", id, selection.len());
            return Ok(selection);
        }
    }
    
    Err("Failed to read from clipboard".to_string())
}

#[tauri::command]
pub fn accept_prediction_word(word: String) -> Result<(), String> {
    let text_to_insert = format!("{} ", word);

    #[cfg(target_os = "windows")]
    {
        let injector = keymind_interceptor_windows::TextInjector::new();
        injector.inject_text(&text_to_insert);
        info!("[Prediction] Accepted next word via TextInjector: '{}'", word);
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(mut cb) = Clipboard::new() {
            if cb.set_text(text_to_insert).is_ok() {
                simulate_paste();
                info!("[Prediction] Accepted next word: '{}'", word);
                return Ok(());
            }
        }
        Err("Failed to write to clipboard".to_string())
    }
}

#[allow(dead_code)]
fn simulate_paste() {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGEventTapLocation};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        const VK_V: u16 = 9;
        if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            if let Ok(event_down) = CGEvent::new_keyboard_event(Some(source.clone()), VK_V, true) {
                event_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                event_down.post(CGEventTapLocation::HID);
            }
            if let Ok(event_up) = CGEvent::new_keyboard_event(Some(source), VK_V, false) {
                event_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                event_up.post(CGEventTapLocation::HID);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        keymind_interceptor_windows::injector::TextInjector::new().simulate_paste();
    }
}

fn simulate_copy() {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGEventTapLocation};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        const VK_C: u16 = 8;
        if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            if let Ok(event_down) = CGEvent::new_keyboard_event(Some(source.clone()), VK_C, true) {
                event_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                event_down.post(CGEventTapLocation::HID);
            }
            if let Ok(event_up) = CGEvent::new_keyboard_event(Some(source), VK_C, false) {
                event_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
                event_up.post(CGEventTapLocation::HID);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        keymind_interceptor_windows::injector::TextInjector::new().simulate_copy();
    }
}
