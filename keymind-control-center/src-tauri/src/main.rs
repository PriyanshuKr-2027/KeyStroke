pub mod copilot;
pub mod shortcuts;

use copilot::{close_palette, copilot_accept, copilot_request, get_selected_text};
use shortcuts::{
    accept_prediction_word, get_shortcuts_list, handle_shortcut_trigger, register_global_shortcuts,
    update_shortcut_binding, ShortcutManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStatus {
    pub engine: String,
    pub ai: String,
    pub grammar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub words_typed: i64,
    pub corrections_made: i64,
    pub variables_used: i64,
    pub ai_requests: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub var_type: String,
    pub value: Option<String>,
    pub ai_prompt: Option<String>,
    pub description: Option<String>,
    pub use_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarStatus {
    pub enabled: bool,
    pub mode: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarFix {
    pub id: String,
    pub original: String,
    pub fixed: String,
    pub rule_id: String,
    pub category: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub app_bundle_id: String,
    pub app_name: String,
    pub autocorrect_enabled: bool,
    pub grammar_enabled: bool,
    pub ai_copilot_enabled: bool,
    pub is_blocked: bool,
}

struct AppState {
    grammar_enabled: Mutex<bool>,
    grammar_mode: Mutex<String>,
}

#[tauri::command]
fn get_engine_status() -> EngineStatus {
    EngineStatus {
        engine: "running".to_string(),
        ai: "connected".to_string(),
        grammar: "ready".to_string(),
    }
}

#[tauri::command]
fn get_stats() -> DailyStats {
    DailyStats {
        words_typed: 4820,
        corrections_made: 142,
        variables_used: 28,
        ai_requests: 12,
    }
}

#[tauri::command]
fn get_variables() -> Vec<Variable> {
    vec![
        Variable {
            key: "phone".to_string(),
            var_type: "static".to_string(),
            value: Some("+1-555-0199".to_string()),
            ai_prompt: None,
            description: Some("Mobile phone number".to_string()),
            use_count: 14,
        },
        Variable {
            key: "date".to_string(),
            var_type: "dynamic".to_string(),
            value: None,
            ai_prompt: None,
            description: Some("Formatted date".to_string()),
            use_count: 8,
        },
        Variable {
            key: "leave".to_string(),
            var_type: "ai".to_string(),
            value: None,
            ai_prompt: Some("Draft a leave application letter...".to_string()),
            description: Some("AI leave request".to_string()),
            use_count: 3,
        },
    ]
}

#[tauri::command]
fn upsert_variable(v: Variable) -> Result<(), String> {
    println!("Upsert variable: {:?}", v);
    Ok(())
}

#[tauri::command]
fn delete_variable(key: String) -> Result<(), String> {
    println!("Delete variable: {}", key);
    Ok(())
}

#[tauri::command]
fn test_variable(key: String) -> String {
    match key.to_lowercase().as_str() {
        "phone" => "+1-555-0199".to_string(),
        "date" => chrono::Local::now().format("%B %d, %Y").to_string(),
        "leave" => "Dear Manager, Please accept this formal leave application...".to_string(),
        _ => format!("Resolved value for /{}", key),
    }
}

#[tauri::command]
fn get_grammar_status(state: tauri::State<AppState>) -> GrammarStatus {
    GrammarStatus {
        enabled: *state.grammar_enabled.lock().unwrap(),
        mode: state.grammar_mode.lock().unwrap().clone(),
        language: "en-US".to_string(),
    }
}

#[tauri::command]
fn toggle_grammar(enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    *state.grammar_enabled.lock().unwrap() = enabled;
    Ok(())
}

#[tauri::command]
fn set_grammar_mode(mode: String, state: tauri::State<AppState>) -> Result<(), String> {
    *state.grammar_mode.lock().unwrap() = mode;
    Ok(())
}

#[tauri::command]
fn get_recent_grammar_fixes() -> Vec<GrammarFix> {
    vec![GrammarFix {
        id: "fix_1".to_string(),
        original: "He are going to teh store.".to_string(),
        fixed: "He is going to the store.".to_string(),
        rule_id: "HE_ARE".to_string(),
        category: "GRAMMAR".to_string(),
        timestamp: "2 min ago".to_string(),
    }]
}

#[tauri::command]
fn get_app_settings() -> Vec<AppSettings> {
    vec![
        AppSettings {
            app_bundle_id: "com.apple.Safari".to_string(),
            app_name: "Safari".to_string(),
            autocorrect_enabled: true,
            grammar_enabled: true,
            ai_copilot_enabled: true,
            is_blocked: false,
        },
        AppSettings {
            app_bundle_id: "com.microsoft.VSCode".to_string(),
            app_name: "Visual Studio Code".to_string(),
            autocorrect_enabled: false,
            grammar_enabled: false,
            ai_copilot_enabled: true,
            is_blocked: false,
        },
    ]
}

#[tauri::command]
fn update_app_settings(s: AppSettings) -> Result<(), String> {
    println!("Updated app settings: {:?}", s);
    Ok(())
}

#[tauri::command]
fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        #[link(name = "ApplicationServices", kind = "framework")]
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }

    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiKeysStatus {
    pub groq_valid: bool,
    pub cerebras_valid: bool,
}

#[tauri::command]
async fn save_api_key(key: String) -> Result<bool, String> {
    let res = save_ai_provider_keys(Some(key), None).await?;
    Ok(res.groq_valid)
}

#[tauri::command]
async fn save_ai_provider_keys(
    groq_key: Option<String>,
    cerebras_key: Option<String>,
) -> Result<AiKeysStatus, String> {
    let client = reqwest::Client::new();
    let mut groq_valid = false;
    let mut cerebras_valid = false;

    let groq_trimmed = groq_key.unwrap_or_default().trim().to_string();
    if !groq_trimmed.is_empty() {
        if let Ok(resp) = client
            .get("https://api.groq.com/openai/v1/models")
            .header("Authorization", format!("Bearer {}", groq_trimmed))
            .send()
            .await
        {
            if resp.status().is_success() {
                groq_valid = true;
            }
        }
    }

    let cerebras_trimmed = cerebras_key.unwrap_or_default().trim().to_string();
    if !cerebras_trimmed.is_empty() {
        if let Ok(resp) = client
            .get("https://api.cerebras.ai/v1/models")
            .header("Authorization", format!("Bearer {}", cerebras_trimmed))
            .send()
            .await
        {
            if resp.status().is_success() {
                cerebras_valid = true;
            }
        }
    }

    if groq_valid || cerebras_valid {
        if let Some(home) = dirs_next::home_dir() {
            let config_dir = home.join(".config").join("keymind");
            let _ = std::fs::create_dir_all(&config_dir);
            let env_path = config_dir.join(".env");
            let mut env_content = String::new();
            if groq_valid {
                env_content.push_str(&format!("GROQ_API_KEY={}\n", groq_trimmed));
            }
            if cerebras_valid {
                env_content.push_str(&format!("CEREBRAS_API_KEY={}\n", cerebras_trimmed));
            }
            let _ = std::fs::write(env_path, env_content);
        }
    }

    Ok(AiKeysStatus {
        groq_valid,
        cerebras_valid,
    })
}

#[tauri::command]
fn get_ai_keys_status() -> AiKeysStatus {
    if let Some(home) = dirs_next::home_dir() {
        let env_path = home.join(".config").join("keymind").join(".env");
        let _ = dotenvy::from_path(env_path);
    }

    let groq_valid = std::env::var("GROQ_API_KEY").map(|k| !k.trim().is_empty()).unwrap_or(false);
    let cerebras_valid = std::env::var("CEREBRAS_API_KEY").map(|k| !k.trim().is_empty()).unwrap_or(false);

    AiKeysStatus {
        groq_valid,
        cerebras_valid,
    }
}

#[tauri::command]
fn install_launch_agent() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let launch_agents_dir = home.join("Library").join("LaunchAgents");
            let _ = std::fs::create_dir_all(&launch_agents_dir);
            let dest_plist = launch_agents_dir.join("com.keymind.engine.plist");

            let plist_content = include_str!("../../../distribution/com.keymind.engine.plist");
            std::fs::write(&dest_plist, plist_content).map_err(|e| e.to_string())?;

            let _ = std::process::Command::new("launchctl")
                .arg("load")
                .arg("-w")
                .arg(&dest_plist)
                .status();
        }
    }
    Ok(())
}

#[tauri::command]
fn uninstall_launch_agent() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let dest_plist = home.join("Library").join("LaunchAgents").join("com.keymind.engine.plist");
            if dest_plist.exists() {
                let _ = std::process::Command::new("launchctl")
                    .arg("unload")
                    .arg(&dest_plist)
                    .status();
                let _ = std::fs::remove_file(dest_plist);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearnedPhraseItem {
    pub id: String,
    pub phrase: String,
    pub frequency: i32,
    pub is_pinned: bool,
    pub app_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalWordItem {
    pub id: String,
    pub word: String,
    pub date_added: String,
}

#[tauri::command]
pub fn get_learned_phrases() -> Vec<LearnedPhraseItem> {
    vec![
        LearnedPhraseItem {
            id: "1".to_string(),
            phrase: "Quarterly financial results".to_string(),
            frequency: 14,
            is_pinned: false,
            app_id: Some("Notes".to_string()),
        },
        LearnedPhraseItem {
            id: "2".to_string(),
            phrase: "Project status update".to_string(),
            frequency: 8,
            is_pinned: true,
            app_id: Some("Slack".to_string()),
        },
        LearnedPhraseItem {
            id: "3".to_string(),
            phrase: "Please find attached document".to_string(),
            frequency: 6,
            is_pinned: false,
            app_id: Some("Mail".to_string()),
        },
    ]
}

#[tauri::command]
pub fn pin_learned_phrase(id: String) -> Result<(), String> {
    info!("Pin phrase triggered for {}", id);
    Ok(())
}

#[tauri::command]
pub fn delete_learned_phrase(id: String) -> Result<(), String> {
    info!("Delete phrase triggered for {}", id);
    Ok(())
}

#[tauri::command]
pub fn clear_all_learned_phrases() -> Result<(), String> {
    info!("Cleared all learned phrases.");
    Ok(())
}

#[tauri::command]
pub fn get_personal_words() -> Vec<PersonalWordItem> {
    vec![
        PersonalWordItem {
            id: "w1".to_string(),
            word: "KeyMind".to_string(),
            date_added: "2026-08-01".to_string(),
        },
        PersonalWordItem {
            id: "w2".to_string(),
            word: "Tauri".to_string(),
            date_added: "2026-08-02".to_string(),
        },
        PersonalWordItem {
            id: "w3".to_string(),
            word: "SymSpell".to_string(),
            date_added: "2026-08-03".to_string(),
        },
    ]
}

#[tauri::command]
pub fn add_personal_word(word: String) -> Result<(), String> {
    info!("Added personal word: {}", word);
    Ok(())
}

#[tauri::command]
pub fn delete_personal_word(id: String) -> Result<(), String> {
    info!("Deleted personal word ID: {}", id);
    Ok(())
}

#[tauri::command]
pub fn toggle_learning_enabled(enabled: bool) -> Result<(), String> {
    info!("Toggled phrase learning to {}", enabled);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            grammar_enabled: Mutex::new(true),
            grammar_mode: Mutex::new("Aggressive".to_string()),
        })
        .manage(ShortcutManager::new())
        .invoke_handler(tauri::generate_handler![
            get_engine_status,
            get_stats,
            get_variables,
            upsert_variable,
            delete_variable,
            test_variable,
            get_grammar_status,
            toggle_grammar,
            set_grammar_mode,
            get_recent_grammar_fixes,
            get_app_settings,
            update_app_settings,
            check_accessibility_permission,
            save_api_key,
            save_ai_provider_keys,
            get_ai_keys_status,
            install_launch_agent,
            uninstall_launch_agent,
            get_selected_text,
            copilot_request,
            copilot_accept,
            close_palette,
            register_global_shortcuts,
            get_shortcuts_list,
            update_shortcut_binding,
            handle_shortcut_trigger,
            accept_prediction_word,
            get_learned_phrases,
            pin_learned_phrase,
            delete_learned_phrase,
            clear_all_learned_phrases,
            get_personal_words,
            add_personal_word,
            delete_personal_word,
            toggle_learning_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
