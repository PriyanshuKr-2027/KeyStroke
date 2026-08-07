use arboard::Clipboard;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::OnceLock;
use tauri::{Manager, Window};

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| reqwest::Client::new())
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamPayload {
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DonePayload {
    pub final_text: String,
}

#[derive(Debug, Serialize)]
struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct GroqStreamRequest {
    model: String,
    messages: Vec<GroqMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GroqDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GroqChoiceChunk {
    delta: GroqDelta,
}

#[derive(Debug, Deserialize)]
struct GroqChunk {
    choices: Vec<GroqChoiceChunk>,
}

pub fn get_system_prompt_for_action(action: &str) -> &'static str {
    match action {
        "rewrite" => "You are a writing assistant. Rewrite the provided text to improve clarity, flow, and readability while preserving the original meaning and tone. Do not add new information. Output ONLY the rewritten text. No preamble, no explanation, no quotes around the output.",
        "grammar" => "You are a grammar correction assistant. Fix all grammar, punctuation, and spelling errors in the provided text. Preserve the author's voice, vocabulary, and sentence structure as much as possible — only fix actual errors. Output ONLY the corrected text. No explanation.",
        "translate" => "Detect the language of the input text and translate it into natural, fluent English.\nOutput format:\n[Translated text here]\nOutput ONLY the translation. No \"Detected language:\" prefix. No explanation.",
        "summarize" => "Summarize the provided text into 2–4 bullet points.\nEach bullet must: start with •, be one sentence, begin with a strong verb, capture a distinct key idea.\nOutput ONLY the bullet points. No preamble. Use plain text, no markdown headers.",
        "expand" => "Expand the provided text by adding relevant detail, examples, or context.\nThe output should be 2–3x longer than the input. Maintain the original tone and voice.\nDo not add fictional claims — only expand with reasonable elaboration.\nOutput ONLY the expanded text.",
        "concise" => "Rewrite the provided text to be as concise as possible without losing meaning.\nTarget 40–60% of the original word count. Remove filler words, redundancy, and padding.\nOutput ONLY the concise version.",
        "professional" => "Rewrite the provided text in a professional, formal tone suitable for business communication.\nFix grammar, elevate vocabulary where appropriate, and remove casual language.\nPreserve all factual content and intent.\nOutput ONLY the rewritten text.",
        "friendly" => "Rewrite the provided text in a warm, conversational, and friendly tone.\nMake it feel approachable and human. Reduce formality where appropriate.\nPreserve all key information.\nOutput ONLY the rewritten text.",
        "continue" => "Continue writing the provided text naturally. Match the existing tone, style, vocabulary, and sentence length. Write 2–4 additional sentences that flow directly from where the text ends. Do not repeat any content from the input.\nOutput ONLY the continuation (not the original text + continuation). Just the new part.",
        "explain" => "Explain the provided text clearly and simply. Assume the reader is intelligent but unfamiliar with the topic. Use plain language and, if helpful, a brief analogy.\nKeep the explanation to 3–5 sentences.\nOutput ONLY the explanation.",
        _ => "You are an AI assistant helping with text editing and refinement. Output ONLY the requested result.",
    }
}

#[tauri::command]
pub fn get_selected_text() -> String {
    Clipboard::new()
        .and_then(|mut c| c.get_text())
        .unwrap_or_else(|_| "".to_string())
}

#[tauri::command]
pub async fn copilot_request(window: Window, text: String, action: String) -> Result<(), String> {
    let (groq_key, cerebras_key) = read_env_keys();

    let mut providers: Vec<(&str, &str, &str)> = Vec::new();
    if !groq_key.trim().is_empty() {
        providers.push(("https://api.groq.com/openai/v1/chat/completions", &groq_key, "llama-3.3-70b-versatile"));
    }
    if !cerebras_key.trim().is_empty() {
        providers.push(("https://api.cerebras.ai/v1/chat/completions", &cerebras_key, "llama3.1-8b"));
    }

    if providers.is_empty() {
        if let Err(e) = window.emit("copilot_error", serde_json::json!({
            "message": "No API keys configured. Go to Settings → AI Providers to add your Groq or Cerebras key."
        })) {
            tracing::warn!("Emit error: {}", e);
        }
        return Err("No API keys configured. Add your API keys in Settings → AI Providers.".to_string());
    }

    let client = get_client();
    let system_prompt = get_system_prompt_for_action(&action);

    let mut res_opt = None;

    for (url, key, model) in &providers {
        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", key)) {
            headers.insert(AUTHORIZATION, val);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let payload = GroqStreamRequest {
            model: model.to_string(),
            messages: vec![
                GroqMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                GroqMessage {
                    role: "user".to_string(),
                    content: text.clone(),
                },
            ],
            max_tokens: 1000,
            temperature: 0.4,
            stream: true,
        };

        let mut backoff = 2u64;
        let mut provider_success = false;

        for _attempt in 0..2 {
            let res = client
                .post(*url)
                .headers(headers.clone())
                .json(&payload)
                .send()
                .await;

            if let Ok(response) = res {
                if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    if let Err(e) = window.emit("rate_limit_warning", format!("AI Provider ({}) rate limited. Retrying...", url)) {
                        tracing::warn!("Emit error: {}", e);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
                    backoff *= 2;
                    continue;
                }

                if response.status().is_success() {
                    res_opt = Some(response);
                    provider_success = true;
                    break;
                }
            }
        }

        if provider_success {
            break;
        } else {
            if let Err(e) = window.emit("rate_limit_warning", format!("Primary provider ({}) unavailable. Failing over to secondary provider...", url)) {
                tracing::warn!("Emit error: {}", e);
            }
        }
    }

    let res = match res_opt {
        Some(r) => r,
        None => {
            let fallback = "[AI unavailable — all configured providers rate-limited or offline]".to_string();
            if let Err(e) = window.emit("copilot_stream", StreamPayload { delta: fallback.clone() }) {
                tracing::warn!("Emit error: {}", e);
            }
            if let Err(e) = window.emit("copilot_done", DonePayload { final_text: fallback }) {
                tracing::warn!("Emit error: {}", e);
            }
            return Ok(());
        }
    };

    let mut stream = res.bytes_stream();
    let mut full_text = String::new();
    let mut buffer = Vec::new();

    while let Some(chunk_res) = stream.next().await {
        if let Ok(bytes) = chunk_res {
            buffer.extend_from_slice(&bytes);
            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = buffer.drain(..=pos).collect::<Vec<u8>>();
                if let Ok(s) = String::from_utf8(line) {
                    let line_trimmed = s.trim();
                    if line_trimmed.starts_with("data: ") {
                        let data_json = line_trimmed.trim_start_matches("data: ");
                        if data_json == "[DONE]" {
                            break;
                        }
                        if let Ok(chunk) = serde_json::from_str::<GroqChunk>(data_json) {
                            if let Some(choice) = chunk.choices.first() {
                                if let Some(ref delta_content) = choice.delta.content {
                                    full_text.push_str(delta_content);
                                    if let Err(e) = window.emit(
                                        "copilot_stream",
                                        StreamPayload {
                                            delta: delta_content.clone(),
                                        },
                                    ) {
                                        tracing::warn!("Emit error: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Err(e) = window.emit("copilot_done", DonePayload { final_text: full_text }) {
        tracing::warn!("Emit error: {}", e);
    }
    Ok(())
}

#[tauri::command]
pub fn copilot_accept(window: Window, final_text: String) -> Result<(), String> {
    // 1. Write final text to clipboard
    if let Ok(mut cb) = Clipboard::new() {
        if let Err(e) = cb.set_text(final_text) {
            tracing::warn!("Failed to set clipboard text: {}", e);
            return Err("Failed to write to clipboard".to_string());
        }
    } else {
        tracing::warn!("Failed to access clipboard");
        return Err("Failed to write to clipboard".to_string());
    }

    // 2. Hide window and sleep slightly to restore focus to previously active app
    let _ = window.hide();
    std::thread::sleep(std::time::Duration::from_millis(50));

    // 3. Simulate paste in previously active app
    simulate_paste();

    // 4. Close window
    let _ = window.close();
    Ok(())
}

#[tauri::command]
pub fn close_palette(window: Window) -> Result<(), String> {
    let _ = window.close();
    Ok(())
}

#[tauri::command]
pub async fn open_palette_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(palette) = app.get_window("palette") {
        if palette.is_visible().unwrap_or(false) {
            let _ = keymind_palette::close_palette(&app);
            return Ok(());
        }
    }
    let context = keymind_palette::capture_context();
    keymind_palette::open_palette(&app, context).await
}

#[tauri::command]
pub async fn run_copilot_prompt(
    prompt: String,
    context_before: String,
    context_after: String,
) -> Result<String, String> {
    let system_prompt = "You are a typing assistant embedded in a desktop productivity app called KeyStroke.\n\
                         The user is actively editing text in another application and has asked for your help.\n\n\
                         Rules:\n\
                         - Reply with ONLY the requested output. No preamble, no explanation, no quotes.\n\
                         - If rewriting, output rewritten text only.\n\
                         - If answering, answer directly and concisely.\n\
                         - Match tone and style of context.\n\
                         - Never add markdown headers or bold text unless context uses it.";

    let formatted_input = if context_before.trim().is_empty() && context_after.trim().is_empty() {
        prompt.clone()
    } else {
        format!(
            "Context (text surrounding cursor):\n[...] {} [CURSOR] {} [...]\n\nTask: {}",
            context_before.trim(),
            context_after.trim(),
            prompt.trim()
        )
    };

    let (groq_key, cerebras_key) = read_env_keys();

    if groq_key.trim().is_empty() && cerebras_key.trim().is_empty() {
        return Err("No API keys configured. Add your API keys in Settings → AI Providers.".to_string());
    }

    let client = get_client();
    let mut providers: Vec<(&str, &str, &str)> = Vec::new();
    if !groq_key.trim().is_empty() {
        providers.push(("https://api.groq.com/openai/v1/chat/completions", &groq_key, "llama-3.3-70b-versatile"));
    }
    if !cerebras_key.trim().is_empty() {
        providers.push(("https://api.cerebras.ai/v1/chat/completions", &cerebras_key, "llama3.1-8b"));
    }

    for (url, key, model) in providers {
        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", key)) {
            headers.insert(AUTHORIZATION, val);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let payload = GroqStreamRequest {
            model: model.to_string(),
            messages: vec![
                GroqMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                GroqMessage {
                    role: "user".to_string(),
                    content: formatted_input.clone(),
                },
            ],
            max_tokens: 1000,
            temperature: 0.3,
            stream: false,
        };

        if let Ok(res) = client.post(url).headers(headers).json(&payload).send().await {
            if res.status().is_success() {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if let Some(content) = json.get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|m| m.get("message"))
                        .and_then(|msg| msg.get("content"))
                        .and_then(|c| c.as_str()) {
                        return Ok(content.trim().to_string());
                    }
                }
            }
        }
    }

    Err("AI processing failed on all configured providers.".to_string())
}

#[tauri::command]
pub async fn inject_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    let _ = keymind_palette::close_palette(&app);
    keymind_palette::inject_text(&text)
}

#[tauri::command]
pub async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    let _ = keymind_palette::close_palette(&app);
    if let Ok(mut cb) = Clipboard::new() {
        cb.set_text(text).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn simulate_paste() {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(50));
    unsafe {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x11), // VK_CONTROL
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x56), // VK_V
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x56),
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x11),
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(target_os = "macos")]
fn simulate_paste() {
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

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn simulate_paste() {
    info!("[Copilot] Paste simulation not implemented for this platform");
}
