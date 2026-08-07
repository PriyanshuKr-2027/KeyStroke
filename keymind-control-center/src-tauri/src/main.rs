#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
use tauri::Manager;
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(default)]
    pub date_of_birth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureToggles {
    pub autocorrect_enabled: bool,
    pub prediction_enabled: bool,
    pub grammar_enabled: bool,
    pub homophone_enabled: bool,
}

impl Default for FeatureToggles {
    fn default() -> Self {
        Self {
            autocorrect_enabled: true,
            prediction_enabled: true,
            grammar_enabled: true,
            homophone_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreData {
    pub profile: UserProfile,
    pub toggles: FeatureToggles,
    pub variables: Vec<Variable>,
    pub personal_words: Vec<PersonalWordItem>,
    pub learned_phrases: Vec<LearnedPhraseItem>,
    pub app_settings: Vec<AppSettings>,
    pub grammar_status: GrammarStatus,
    pub daily_stats: DailyStats,
    pub grammar_fixes: Vec<GrammarFix>,
}

impl Default for StoreData {
    fn default() -> Self {
        Self {
            profile: UserProfile::default(),
            toggles: FeatureToggles::default(),
            variables: vec![],
            personal_words: vec![],
            learned_phrases: vec![],
            app_settings: vec![],
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
            grammar_fixes: vec![],
        }
    }
}

fn get_store_path() -> Option<std::path::PathBuf> {
    dirs_next::home_dir().map(|h| h.join(".config").join("keystroke").join("store.json"))
}

fn load_store() -> StoreData {
    if let Some(path) = get_store_path() {
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str::<StoreData>(&content) {
                    Ok(data) => return data,
                    Err(e) => {
                        tracing::warn!("Failed to parse store file: {}", e);
                        let bak_path = path.with_extension("json.bak");
                        let _ = std::fs::rename(&path, &bak_path);
                    }
                },
                Err(e) => tracing::warn!("Failed to read store file: {}", e),
            }
        }
    }
    StoreData::default()
}

fn save_store(store: &StoreData) {
    if let Some(path) = get_store_path() {
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                tracing::warn!("Failed to create store directory: {}", e);
            }
        }
        match serde_json::to_string_pretty(store) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, json) {
                    tracing::warn!("Failed to write store file: {}", e);
                }
            }
            Err(e) => tracing::warn!("Failed to serialize store data: {}", e),
        }
    }
}

fn save_store_async(store: StoreData) {
    tokio::spawn(async move {
        tokio::task::spawn_blocking(move || {
            save_store(&store);
        })
        .await
        .ok();
    });
}

pub struct AppState {
    pub store: Mutex<StoreData>,
    /// Session-only rejection memory: words the user deliberately backspaced after
    /// autocorrect fired. These will never be auto-corrected again for this session.
    pub user_correction_overrides: Mutex<std::collections::HashSet<String>>,
}

#[tauri::command]
fn get_engine_status() -> EngineStatus {
    let running = keymind_interceptor_windows::is_interceptor_active();
    EngineStatus {
        engine: if running { "running".to_string() } else { "stopped".to_string() },
        ai: "connected".to_string(),
        grammar: "ready".to_string(),
    }
}

#[tauri::command]
fn toggle_engine_state(running: bool) -> EngineStatus {
    keymind_interceptor_windows::set_interceptor_active(running);
    get_engine_status()
}

#[tauri::command]
fn get_stats(state: tauri::State<AppState>) -> DailyStats {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).daily_stats.clone()
}

#[tauri::command]
fn get_variables(state: tauri::State<AppState>) -> Vec<Variable> {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).variables.clone()
}

#[tauri::command]
fn upsert_variable(v: Variable, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
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
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.variables.retain(|v| v.key != key);
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn test_variable(key: String, state: tauri::State<AppState>) -> String {
    let store = state.store.lock().unwrap_or_else(|e| e.into_inner());
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
    state.store.lock().unwrap_or_else(|e| e.into_inner()).grammar_status.clone()
}

#[tauri::command]
fn toggle_grammar(enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.grammar_status.enabled = enabled;
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn set_grammar_mode(mode: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.grammar_status.mode = mode;
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn get_recent_grammar_fixes(state: tauri::State<AppState>) -> Vec<GrammarFix> {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).grammar_fixes.clone()
}

#[tauri::command]
fn get_app_settings(state: tauri::State<AppState>) -> Vec<AppSettings> {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).app_settings.clone()
}

#[tauri::command]
fn update_app_settings(s: AppSettings, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
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

#[tauri::command]
fn open_accessibility_settings() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "ms-settings:privacy-accessibility"])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn();
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
            let config_dir = home.join(".config").join("keystroke");
            let _ = std::fs::create_dir_all(&config_dir);
            let env_path = config_dir.join(".env");
            let existing = std::fs::read_to_string(&env_path).unwrap_or_default();
            let mut lines: Vec<String> = existing.lines().map(|s| s.to_string()).collect();
            
            let mut groq_updated = false;
            let mut cerebras_updated = false;
            
            for line in &mut lines {
                if groq_valid && line.starts_with("GROQ_API_KEY=") {
                    *line = format!("GROQ_API_KEY={}", groq_trimmed);
                    groq_updated = true;
                }
                if cerebras_valid && line.starts_with("CEREBRAS_API_KEY=") {
                    *line = format!("CEREBRAS_API_KEY={}", cerebras_trimmed);
                    cerebras_updated = true;
                }
            }
            
            if groq_valid && !groq_updated {
                lines.push(format!("GROQ_API_KEY={}", groq_trimmed));
            }
            if cerebras_valid && !cerebras_updated {
                lines.push(format!("CEREBRAS_API_KEY={}", cerebras_trimmed));
            }
            
            let _ = std::fs::write(env_path, lines.join("\n"));
        }
    }

    Ok(AiKeysStatus {
        groq_valid,
        cerebras_valid,
    })
}

fn read_env_keys() -> (String, String) {
    let env_path = dirs_next::home_dir()
        .map(|h| h.join(".config").join("keystroke").join(".env"))
        .unwrap_or_default();
    let content = std::fs::read_to_string(&env_path).unwrap_or_default();
    let mut groq = String::new();
    let mut cerebras = String::new();
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("GROQ_API_KEY=") {
            groq = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("CEREBRAS_API_KEY=") {
            cerebras = val.trim().to_string();
        }
    }
    (groq, cerebras)
}

#[tauri::command]
fn get_ai_keys_status() -> AiKeysStatus {
    let (groq, cerebras) = read_env_keys();
    AiKeysStatus {
        groq_valid: !groq.trim().is_empty(),
        cerebras_valid: !cerebras.trim().is_empty(),
    }
}

#[tauri::command]
fn install_launch_agent() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let launch_agents_dir = home.join("Library").join("LaunchAgents");
            std::fs::create_dir_all(&launch_agents_dir).map_err(|e| format!("Failed to create launch agents dir: {}", e))?;
            let dest_plist = launch_agents_dir.join("com.keystroke.engine.plist");

            let plist_content = include_str!("../../../distribution/com.keymind.engine.plist");
            std::fs::write(&dest_plist, plist_content).map_err(|e| format!("Failed to write plist: {}", e))?;

            std::process::Command::new("launchctl")
                .arg("load")
                .arg("-w")
                .arg(&dest_plist)
                .status()
                .map_err(|e| format!("Failed to load launch agent: {}", e))?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy().to_string();
            std::process::Command::new("reg")
                .args(&[
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "KeyStroke",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\" --autostart", exe_str),
                    "/f",
                ])
                .status()
                .map_err(|e| format!("Failed to add registry key: {}", e))?;
        }
    }
    Ok(())
}

#[tauri::command]
fn uninstall_launch_agent() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let dest_plist = home.join("Library").join("LaunchAgents").join("com.keystroke.engine.plist");
            if dest_plist.exists() {
                std::process::Command::new("launchctl")
                    .arg("unload")
                    .arg(&dest_plist)
                    .status()
                    .map_err(|e| format!("Failed to unload launch agent: {}", e))?;
                std::fs::remove_file(dest_plist).map_err(|e| format!("Failed to remove plist: {}", e))?;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("reg")
            .args(&[
                "delete",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                "KeyStroke",
                "/f",
            ])
            .status()
            .map_err(|e| format!("Failed to delete registry key: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn get_learned_phrases(state: tauri::State<AppState>) -> Vec<LearnedPhraseItem> {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).learned_phrases.clone()
}

#[tauri::command]
fn pin_learned_phrase(id: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(p) = store.learned_phrases.iter_mut().find(|x| x.id == id) {
        p.is_pinned = !p.is_pinned;
    }
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn delete_learned_phrase(id: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.learned_phrases.retain(|x| x.id != id);
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn clear_all_learned_phrases(state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.learned_phrases.clear();
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn get_personal_words(state: tauri::State<AppState>) -> Vec<PersonalWordItem> {
    state.store.lock().unwrap_or_else(|e| e.into_inner()).personal_words.clone()
}

#[tauri::command]
fn add_personal_word(word: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
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
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.personal_words.retain(|w| w.id != id && w.word != id);
    save_store(&store);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocorrectResult {
    pub original: String,
    pub corrected: String,
    pub confidence: f32,
}

use keymind_autocorrect::bigram::BigramModel;
use keymind_autocorrect::symspell_layer::SymSpellEngine;
use keymind_autocorrect::TrigramPredictor;
use std::sync::OnceLock;

static SYMSPELL: OnceLock<SymSpellEngine> = OnceLock::new();
static PREDICTOR: OnceLock<TrigramPredictor> = OnceLock::new();
static BIGRAM_MODEL: OnceLock<BigramModel> = OnceLock::new();

fn get_symspell() -> &'static SymSpellEngine {
    SYMSPELL.get_or_init(|| SymSpellEngine::new())
}

fn get_predictor() -> &'static TrigramPredictor {
    PREDICTOR.get_or_init(|| TrigramPredictor::new())
}

fn get_bigram_model() -> &'static BigramModel {
    BIGRAM_MODEL.get_or_init(|| BigramModel::new())
}

#[tauri::command]
fn check_autocorrect_word(word: String) -> Option<AutocorrectResult> {
    let clean = word.trim();
    if clean.is_empty() {
        return None;
    }

    let sym = get_symspell();
    if let Some((suggested, conf)) = sym.check(clean) {
        return Some(AutocorrectResult {
            original: word,
            corrected: suggested,
            confidence: conf,
        });
    }

    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionResult {
    pub candidate_word: String,
    pub suggestions: Vec<String>,
}

#[tauri::command]
fn predict_next_word(context: String) -> Option<PredictionResult> {
    let clean_context = context.trim();
    if clean_context.is_empty() {
        return None;
    }

    let predictor = get_predictor();
    let suggestions = predictor.predict(clean_context);

    if !suggestions.is_empty() {
        Some(PredictionResult {
            candidate_word: suggestions[0].clone(),
            suggestions,
        })
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrammarCheckResult {
    pub original: String,
    pub fixed: String,
    pub issues: Vec<String>,
}

static GRAMMAR_ENGINE: OnceLock<keymind_grammar::GrammarEngine> = OnceLock::new();

fn get_grammar_engine() -> &'static keymind_grammar::GrammarEngine {
    GRAMMAR_ENGINE.get_or_init(|| keymind_grammar::GrammarEngine::new(8081))
}

#[tauri::command]
async fn check_grammar_text(text: String, state: tauri::State<'_, AppState>) -> Result<GrammarCheckResult, String> {
    let clean = text.trim();
    if clean.is_empty() {
        return Ok(GrammarCheckResult {
            original: text,
            fixed: String::new(),
            issues: vec![],
        });
    }

    let engine = get_grammar_engine();
    let issues_list = engine.check_text(clean, "en-US").await;
    let fixed_text = engine.fix_text(clean).await;

    let issue_messages: Vec<String> = issues_list.iter().map(|i| i.message.clone()).collect();

    if !issues_list.is_empty() {
        let new_fixes: Vec<GrammarFix> = issues_list
            .into_iter()
            .map(|i| GrammarFix {
                id: format!("gf_{}", chrono::Utc::now().timestamp_millis()),
                original: clean.to_string(),
                fixed: fixed_text.clone(),
                rule_id: i.rule_id,
                category: i.category,
                timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect();

        let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
        store.grammar_fixes.extend(new_fixes);
        save_store(&store);
    }

    Ok(GrammarCheckResult {
        original: text,
        fixed: fixed_text,
        issues: issue_messages,
    })
}

#[tauri::command]
fn toggle_learning_enabled(enabled: bool) -> Result<(), String> {
    info!("Toggled phrase learning to {}", enabled);
    Ok(())
}

#[tauri::command]
fn set_grammar_language(language: String, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.grammar_status.language = language;
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn toggle_engine_component(_engine: String, _enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    if _engine == "grammar" {
        store.grammar_status.enabled = _enabled;
    }
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn set_grammar_sensitivity(_level: u32, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.grammar_status.mode = if _level > 50 { "Aggressive".to_string() } else { "Standard".to_string() };
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn get_feature_toggles(state: tauri::State<AppState>) -> FeatureToggles {
    let store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.toggles.clone()
}

#[tauri::command]
fn update_feature_toggle(feature: String, enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    match feature.as_str() {
        "autocorrect" => store.toggles.autocorrect_enabled = enabled,
        "prediction" => store.toggles.prediction_enabled = enabled,
        "grammar" => store.toggles.grammar_enabled = enabled,
        "homophone" => store.toggles.homophone_enabled = enabled,
        _ => {}
    }
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn save_profile(first_name: String, last_name: String, email: String, date_of_birth: Option<String>, state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.profile = UserProfile {
        first_name,
        last_name,
        email,
        date_of_birth: date_of_birth.unwrap_or_default(),
    };
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn set_typing_preset(preset: String) -> Result<(), String> {
    tracing::info!("Typing preset set to: {}", preset);
    Ok(())
}

#[tauri::command]
fn export_local_data(state: tauri::State<AppState>) -> Result<String, String> {
    let store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    serde_json::to_string_pretty(&*store).map_err(|e| format!("Export failed: {}", e))
}

#[tauri::command]
fn clear_activity_history(state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    store.grammar_fixes = vec![];
    store.daily_stats = DailyStats { words_typed: 0, corrections_made: 0, variables_used: 0, ai_requests: 0 };
    save_store(&store);
    Ok(())
}

#[tauri::command]
fn purge_database(state: tauri::State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
    *store = StoreData::default();
    save_store(&store);
    Ok(())
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Reduce WebView2 RAM footprint by collapsing renderer processes, disabling dedicated GPU process, and capping V8 GC memory
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--disable-gpu --disable-gpu-compositing --renderer-process-limit=1 --js-flags=\"--max-old-space-size=48\" --disable-site-isolation-trials"
        );
    }

    let store_data = load_store();

    tauri::Builder::default()
        .setup(|app| {
            let is_autostart = std::env::args().any(|a| a == "--autostart" || a == "--minimized");
            if is_autostart {
                use tauri::Manager;
                if let Some(window) = app.get_window("main") {
                    let _ = window.hide();
                }
            }

            #[cfg(target_os = "windows")]
            {
                // Spawn background memory trimmer to return unused committed pages to Windows OS every 30s
                tauri::async_runtime::spawn(async move {
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                        unsafe {
                            use windows_sys::Win32::System::Threading::{GetCurrentProcess, SetProcessWorkingSetSize};
                            SetProcessWorkingSetSize(GetCurrentProcess(), usize::MAX, usize::MAX);
                        }
                    }
                });

                let (tx, mut rx) = tokio::sync::mpsc::channel(64);
                std::thread::spawn(move || {
                    let _ = keymind_interceptor_windows::lifecycle::start_interceptor(tx);
                });

                let app_handle = app.handle();
                tauri::async_runtime::spawn(async move {
                    while let Some(event) = rx.recv().await {
                        match event {
                            keymind_interceptor_windows::Event::PaletteRequested => {
                                let _ = open_palette_window(app_handle.clone()).await;
                            }
                            keymind_interceptor_windows::Event::HotKeyTriggered(id) => {
                                let shortcut_name = match id {
                                    1 => "copilot_palette",
                                    2 => "grammar_fix",
                                    3 => "copilot_professional",
                                    4 => "copilot_summarize",
                                    5 => "ai_expand",
                                    6 => "toggle_engine",
                                    _ => "",
                                };
                                if shortcut_name == "copilot_palette" {
                                    let _ = open_palette_window(app_handle.clone()).await;
                                } else if !shortcut_name.is_empty() {
                                    let _ = handle_shortcut_trigger(shortcut_name.to_string()).await;
                                }
                            }
                            keymind_interceptor_windows::Event::WordCompleted { word, context } => {
                                let state = app_handle.state::<AppState>();

                                // 1. Check personal dictionary whitelist
                                let is_whitelisted = {
                                    let store = state.store.lock().unwrap_or_else(|e| e.into_inner());
                                    store.personal_words.iter().any(|w| w.word.eq_ignore_ascii_case(&word))
                                };
                                if is_whitelisted {
                                    return;
                                }

                                // 2. Check custom variables (Enforcing '/' prefix for variable expansion)
                                let (replacement, autocorrect_enabled, prediction_enabled, grammar_enabled) = {
                                    let store = state.store.lock().unwrap_or_else(|e| e.into_inner());
                                    
                                    let rep = if word.starts_with('/') {
                                        let clean_key = &word[1..]; // e.g. "email" from "/email"
                                        store.variables.iter().find(|v| {
                                            v.key.eq_ignore_ascii_case(&word)
                                                || v.key.eq_ignore_ascii_case(clean_key)
                                                || format!("/{}", v.key).eq_ignore_ascii_case(&word)
                                        }).and_then(|v| v.value.clone())
                                    } else {
                                        None
                                    };

                                    (
                                        rep,
                                        store.toggles.autocorrect_enabled,
                                        store.toggles.prediction_enabled,
                                        store.toggles.grammar_enabled,
                                    )
                                };

                                // 2b. Rejection memory guard — user previously backspaced
                                // a correction for this exact word, so respect their choice
                                // and skip autocorrect entirely for this word this session.
                                let is_user_rejected = {
                                    let overrides = state.user_correction_overrides.lock().unwrap_or_else(|e| e.into_inner());
                                    overrides.contains(&word.to_lowercase())
                                };

                                if let Some(rep) = replacement {
                                    let injector = keymind_interceptor_windows::TextInjector::new();
                                    injector.send_backspaces(word.len() + 1);
                                    let mut expanded = rep;
                                    if expanded.contains("{date}") {
                                        expanded = expanded.replace("{date}", &chrono::Local::now().format("%B %d, %Y").to_string());
                                    }
                                    if expanded.contains("{time}") {
                                        expanded = expanded.replace("{time}", &chrono::Local::now().format("%H:%M:%S").to_string());
                                    }
                                    if expanded.contains("{clipboard}") {
                                        let clip_text = arboard::Clipboard::new()
                                            .and_then(|mut c| c.get_text())
                                            .unwrap_or_default();
                                        expanded = expanded.replace("{clipboard}", &clip_text);
                                    }
                                    injector.inject_text(&expanded);
                                    // Bug 3 fix: Clear buffer so next word starts fresh.
                                    // Without this, backspace+retype after a variable
                                    // expansion would not re-trigger the substitution.
                                    keymind_interceptor_windows::clear_word_buffer();
                                    let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
                                    store.daily_stats.variables_used += 1;
                                    save_store_async(store.clone());
                                } else {
                                    let mut corrected_typo = false;

                                    // 3. Multi-layered SymSpell + Bigram Context Re-ranking
                                    // Skip if user previously rejected a correction for this exact word.
                                    if autocorrect_enabled && !is_user_rejected {
                                        let sym = get_symspell();
                                        let bigram = get_bigram_model();
                                        if let Some((suggested, base_conf)) = sym.check(&word) {
                                            let prev_word = context.split_whitespace().last().unwrap_or("");
                                            let mut final_conf = base_conf;

                                            if !prev_word.is_empty() {
                                                if let Some(bg_score) = bigram.score(prev_word, &suggested) {
                                                    final_conf = (base_conf * 0.6 + bg_score * 0.4).min(0.99);
                                                }
                                            }

                                            if final_conf >= 0.70 {
                                                let injector = keymind_interceptor_windows::TextInjector::new();
                                                injector.send_backspaces(word.len());
                                                injector.inject_text(&suggested);
                                                // Bug 3 fix: Clear buffer so fresh typing after this
                                                // correction is treated as a new word.
                                                keymind_interceptor_windows::clear_word_buffer();

                                                // Rejection memory: record this word so if the user
                                                // backspaces the correction and retypes it, we leave
                                                // it alone — they clearly don't want it changed.
                                                {
                                                    let mut overrides = state.user_correction_overrides.lock().unwrap_or_else(|e| e.into_inner());
                                                    overrides.insert(word.to_lowercase());
                                                }

                                                let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
                                                store.daily_stats.corrections_made += 1;
                                                store.grammar_fixes.push(GrammarFix {
                                                    id: format!("gf_{}", chrono::Utc::now().timestamp_millis()),
                                                    original: word.clone(),
                                                    fixed: suggested.clone(),
                                                    rule_id: "symspell_bigram".to_string(),
                                                    category: "TYPOS".to_string(),
                                                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                                });
                                                save_store_async(store.clone());
                                                corrected_typo = true;
                                            }
                                        }
                                    }

                                    // 4. Live Next-Word Prediction Trigger
                                    if !corrected_typo && prediction_enabled {
                                        let predictor = get_predictor();
                                        let predictions = predictor.predict(&context);
                                        if !predictions.is_empty() {
                                            let _ = app_handle.emit_all("prediction-update", serde_json::json!({
                                                "candidate": predictions[0],
                                                "suggestions": predictions,
                                            }));
                                        }
                                    }

                                    // 5. Live Sentence Grammar Assistant (nlprule)
                                    // Bug 2 fix: Only run grammar engine on a REAL sentence context.
                                    // The WORD_BUFFER sends `word` as both word AND context, meaning
                                    // context == the single typed word. Running nlprule on a single
                                    // word produces garbage ("grammar" -> "gamer").
                                    // Gate: require >= 4 whitespace-delimited words AND sentence-ending
                                    // punctuation before invoking the grammar engine.
                                    let context_word_count = context.split_whitespace().count();
                                    if !corrected_typo
                                        && grammar_enabled
                                        && context_word_count >= 4
                                        && (context.ends_with('.') || context.ends_with('!') || context.ends_with('?'))
                                    {
                                        let engine = get_grammar_engine();
                                        let fixed_sentence = engine.fix_text(&context).await;
                                        if !fixed_sentence.is_empty() && fixed_sentence != context {
                                            let injector = keymind_interceptor_windows::TextInjector::new();
                                            injector.send_backspaces(context.len());
                                            injector.inject_text(&fixed_sentence);
                                            // Bug 3 fix: Clear buffer after grammar injection
                                            keymind_interceptor_windows::clear_word_buffer();

                                            let mut store = state.store.lock().unwrap_or_else(|e| e.into_inner());
                                            store.daily_stats.corrections_made += 1;
                                            store.grammar_fixes.push(GrammarFix {
                                                id: format!("gf_{}", chrono::Utc::now().timestamp_millis()),
                                                original: context.clone(),
                                                fixed: fixed_sentence.clone(),
                                                rule_id: "nlprule_grammar".to_string(),
                                                category: "GRAMMAR".to_string(),
                                                timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                            });
                                            save_store_async(store.clone());
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                });
            }
            Ok(())
        })
        .manage(AppState {
            store: Mutex::new(store_data),
            user_correction_overrides: Mutex::new(std::collections::HashSet::new()),
        })
        .manage(ShortcutManager::new())
        .invoke_handler(tauri::generate_handler![
            get_engine_status,
            toggle_engine_state,
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
            open_accessibility_settings,
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
            toggle_learning_enabled,
            check_autocorrect_word,
            predict_next_word,
            check_grammar_text,
            set_grammar_language,
            toggle_engine_component,
            set_grammar_sensitivity,
            save_profile,
            get_feature_toggles,
            update_feature_toggle,
            set_typing_preset,
            export_local_data,
            clear_activity_history,
            purge_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
