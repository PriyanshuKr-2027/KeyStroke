use arboard::Clipboard;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use tauri::Window;
use tracing::info;

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
        .unwrap_or_else(|_| "Sample text for AI Copilot action...".to_string())
}

#[tauri::command]
pub async fn copilot_request(window: Window, text: String, action: String) -> Result<(), String> {
    if let Some(home) = dirs_next::home_dir() {
        let env_path = home.join(".config").join("keymind").join(".env");
        let _ = dotenvy::from_path(env_path);
    }

    let groq_key = env::var("GROQ_API_KEY").unwrap_or_default();
    let cerebras_key = env::var("CEREBRAS_API_KEY").unwrap_or_default();

    let mut providers: Vec<(&str, &str, &str)> = Vec::new();
    if !groq_key.trim().is_empty() {
        providers.push(("https://api.groq.com/openai/v1/chat/completions", &groq_key, "llama-3.3-70b-versatile"));
    }
    if !cerebras_key.trim().is_empty() {
        providers.push(("https://api.cerebras.ai/v1/chat/completions", &cerebras_key, "llama3.1-8b"));
    }

    if providers.is_empty() {
        // Fallback simulated streaming for testing without live API keys
        let mock_text = match action.as_str() {
            "rewrite" => "The team meeting roadmap should be updated prior to discussion.",
            "grammar" => "The project roadmap needs to be updated before the upcoming team meeting.",
            "summarize" => "• Update roadmap\n• Review before team meeting",
            _ => "This is a simulated AI Copilot response for testing.",
        };

        for word in mock_text.split_whitespace() {
            let delta = format!("{} ", word);
            let _ = window.emit("copilot_stream", StreamPayload { delta });
            tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        }

        let _ = window.emit("copilot_done", DonePayload { final_text: mock_text.to_string() });
        return Ok(());
    }

    let client = reqwest::Client::new();
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
                    let _ = window.emit("rate_limit_warning", format!("AI Provider ({}) rate limited. Retrying...", url));
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
            let _ = window.emit("rate_limit_warning", format!("Primary provider ({}) unavailable. Failing over to secondary provider...", url));
        }
    }

    let res = match res_opt {
        Some(r) => r,
        None => {
            let fallback = "[AI unavailable — all configured providers rate-limited or offline]".to_string();
            let _ = window.emit("copilot_stream", StreamPayload { delta: fallback.clone() });
            let _ = window.emit("copilot_done", DonePayload { final_text: fallback });
            return Ok(());
        }
    };

    let mut stream = res.bytes_stream();
    let mut full_text = String::new();

    while let Some(chunk_res) = stream.next().await {
        if let Ok(bytes) = chunk_res {
            let s = String::from_utf8_lossy(&bytes);
            for line in s.lines() {
                let line_trimmed = line.trim();
                if line_trimmed.starts_with("data: ") {
                    let data_json = line_trimmed.trim_start_matches("data: ");
                    if data_json == "[DONE]" {
                        break;
                    }
                    if let Ok(chunk) = serde_json::from_str::<GroqChunk>(data_json) {
                        if let Some(choice) = chunk.choices.first() {
                            if let Some(ref delta_content) = choice.delta.content {
                                full_text.push_str(delta_content);
                                let _ = window.emit(
                                    "copilot_stream",
                                    StreamPayload {
                                        delta: delta_content.clone(),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = window.emit("copilot_done", DonePayload { final_text: full_text });
    Ok(())
}

#[tauri::command]
pub fn copilot_accept(window: Window, final_text: String) -> Result<(), String> {
    // 1. Write final text to clipboard
    if let Ok(mut cb) = Clipboard::new() {
        let _ = cb.set_text(final_text);
    }

    // 2. Simulate paste in previously active app
    simulate_paste();

    // 3. Close window
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
    let context = keymind_palette::capture_context();
    keymind_palette::open_palette(&app, context).await
}

#[tauri::command]
pub async fn run_copilot_prompt(
    prompt: String,
    context_before: String,
    context_after: String,
) -> Result<String, String> {
    let system_prompt = "You are a typing assistant embedded in a desktop productivity app called KeyMind.\n\
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

    if let Some(home) = dirs_next::home_dir() {
        let env_path = home.join(".config").join("keymind").join(".env");
        let _ = dotenvy::from_path(env_path);
    }

    let groq_key = env::var("GROQ_API_KEY").unwrap_or_default();
    let cerebras_key = env::var("CEREBRAS_API_KEY").unwrap_or_default();

    if groq_key.trim().is_empty() && cerebras_key.trim().is_empty() {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        return Ok(format!("Simulated result for: {}", prompt));
    }

    let client = reqwest::Client::new();
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
                    if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
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

    #[cfg(not(target_os = "macos"))]
    {
        info!("[Copilot] Simulated Paste event");
    }
}
