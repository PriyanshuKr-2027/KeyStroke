pub mod copilot;
pub mod shortcuts;

use copilot::{
    close_palette, copy_to_clipboard, copilot_accept, copilot_request, get_selected_text,
    inject_text, open_palette_window, run_copilot_prompt,
};
use shortcuts::{
    accept_prediction_word, get_shortcuts_list, handle_shortcut_trigger, register_global_shortcuts,
    update_shortcut_binding, ShortcutManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::info;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPhraseItem {
    pub id: String,
    pub phrase: String,
    pub frequency: i32,
    pub is_pinned: bool,
    pub app_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalWordItem {
    pub id: String,
    pub word: String,
    pub date_added: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreData {
    pub variables: Vec<Variable>,
    pub personal_words: Vec<PersonalWordItem>,
    pub learned_phrases: Vec<LearnedPhraseItem>,
    pub app_settings: Vec<AppSettings>,
    pub grammar_status: GrammarStatus,
    pub daily_stats: DailyStats,
}

impl Default for StoreData {
    fn default() -> Self {
        Self {
            variables: vec![
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
            ],
            personal_words: vec![
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
            ],
            learned_phrases: vec![],
            app_settings: vec![
                AppSettings {
                    app_bundle_id: "com.microsoft.VSCode".to_string(),
                    app_name: "VS Code".to_string(),
                    autocorrect_enabled: true,
                    grammar_enabled: true,
                    ai_copilot_enabled: true,
                    is_blocked: false,
                },
                AppSettings {
                    app_bundle_id: "com.slack.Slack".to_string(),
                    app_name: "Slack".to_string(),
                    autocorrect_enabled: true,
                    grammar_enabled: true,
                    ai_copilot_enabled: true,
                    is_blocked: false,
                },
            ],
            grammar_status: GrammarStatus {
                enabled: true,
                mode: "Aggressive".to_string(),
                language: "en-US".to_string(),
            },
            daily_stats: DailyStats {
                words_typed: 0,
                corrections_made: 0,
                variables_used: 0,
                ai_requests: 0,
            },
        }
    }
}

fn get_store_path() -> Option<std::path::PathBuf> {
    dirs_next::home_dir().map(|h| h.join(".config").join("keymind").join("store.json"))
}

fn load_store() -> StoreData {
    if let Some(path) = get_store_path() {
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(data) = serde_json::from_str::<StoreData>(&content) {
                    return data;
                }
            }
        }
    }
    StoreData::default()
}

fn save_store(store: &StoreData) {
    if let Some(path) = get_store_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(store) {
            let _ = std::fs::write(path, json);
        }
    }
}

pub struct AppState {
    pub store: Mutex<StoreData>,
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
fn get_stats(state: tauri::State<AppState>) -> DailyStats {
    state.store.lock().unwrap().daily_stats.clone()
}

#[tauri::command]
fn get_variables(state: tauri::State<AppState>) -> Vec<Variable> {
    state.store.lock().unwrap().variables.clone()
}

#[tauri::command]
fn upsert_variable(v: Variable, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    let idx = store.variables.iter().position(|x| x.key == v.key);
    if let Some(i) = idx {
        store.variables[i] = v;
    } else {
        store.variables.push(v);
    }
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn delete_variable(key: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.variables.retain(|v| v.key != key);
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn test_variable(key: String, state: tauri::State<AppState>) -> String {
    let store = state.store.lock().unwrap();
    if let Some(v) = store.variables.iter().find(|x| x.key.eq_ignore_ascii_case(&key)) {
        if v.var_type == "static" {
            return v.value.clone().unwrap_or_default();
        } else if v.var_type == "dynamic" {
            return chrono::Local::now().format("%B %d, %Y").to_string();
        } else if v.var_type == "ai" {
            return v.ai_prompt.clone().unwrap_or_else(|| "AI prompt output sample".to_string());
        }
    }

    match key.to_lowercase().as_str() {
        "date" => chrono::Local::now().format("%B %d, %Y").to_string(),
        "time" => chrono::Local::now().format("%H:%M:%S").to_string(),
        _ => format!("Resolved value for /{}", key),
    }
}

#[tauri::command]
fn get_grammar_status(state: tauri::State<AppState>) -> GrammarStatus {
    state.store.lock().unwrap().grammar_status.clone()
}

#[tauri::command]
fn toggle_grammar(enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.grammar_status.enabled = enabled;
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn set_grammar_mode(mode: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.grammar_status.mode = mode;
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn get_recent_grammar_fixes() -> Vec<GrammarFix> {
    vec![]
}

#[tauri::command]
fn get_app_settings(state: tauri::State<AppState>) -> Vec<AppSettings> {
    state.store.lock().unwrap().app_settings.clone()
}

#[tauri::command]
fn update_app_settings(s: AppSettings, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    let idx = store.app_settings.iter().position(|x| x.app_bundle_id == s.app_bundle_id);
    if let Some(i) = idx {
        store.app_settings[i] = s;
    } else {
        store.app_settings.push(s);
    }
    save_store(&store);
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

    #[cfg(target_os = "windows")]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy().to_string();
            let _ = std::process::Command::new("reg")
                .args(&[
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "KeyMind",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\"", exe_str),
                    "/f",
                ])
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

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("reg")
            .args(&[
                "delete",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                "KeyMind",
                "/f",
            ])
            .status();
    }
    Ok(())
}

#[tauri::command]
fn get_learned_phrases(state: tauri::State<AppState>) -> Vec<LearnedPhraseItem> {
    state.store.lock().unwrap().learned_phrases.clone()
}

#[tauri::command]
fn pin_learned_phrase(id: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    if let Some(p) = store.learned_phrases.iter_mut().find(|x| x.id == id) {
        p.is_pinned = !p.is_pinned;
    }
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn delete_learned_phrase(id: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.learned_phrases.retain(|x| x.id != id);
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn clear_all_learned_phrases(state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.learned_phrases.clear();
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn get_personal_words(state: tauri::State<AppState>) -> Vec<PersonalWordItem> {
    state.store.lock().unwrap().personal_words.clone()
}

#[tauri::command]
fn add_personal_word(word: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    let clean = word.trim().to_string();
    if !clean.is_empty() && !store.personal_words.iter().any(|w| w.word.eq_ignore_ascii_case(&clean)) {
        let item = PersonalWordItem {
            id: format!("w_{}", chrono::Utc::now().timestamp_millis()),
            word: clean,
            date_added: chrono::Local::now().format("%Y-%m-%d").to_string(),
        };
        store.personal_words.push(item);
        save_store(&store);
    }
    Ok(())
}

#[tauri::command]
fn delete_personal_word(id: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.personal_words.retain(|w| w.id != id && w.word != id);
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn toggle_learning_enabled(enabled: bool) -> Result<(), String> {
    info!("Toggled phrase learning to {}", enabled);
    Ok(())
}

fn main() {
    let store_data = load_store();

    tauri::Builder::default()
        .manage(AppState {
            store: Mutex::new(store_data),
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
            open_palette_window,
            run_copilot_prompt,
            inject_text,
            copy_to_clipboard,
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
