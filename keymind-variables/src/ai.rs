use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;
use tokio::time::timeout;

#[derive(Error, Debug)]
pub enum AiError {
    #[error("Missing GROQ_API_KEY environment variable")]
    MissingApiKey,
    #[error("Groq API request timed out (5s limit)")]
    Timeout,
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error response: {0}")]
    ApiError(String),
}

#[derive(Debug, Serialize)]
struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: GroqMessageResponse,
}

#[derive(Debug, Deserialize)]
struct GroqMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
}

#[async_trait]
pub trait GroqClientTrait: Send + Sync {
    async fn generate(&self, system_prompt: &str, user_content: &str) -> Result<String, AiError>;
}

pub struct GroqClient {
    api_key: String,
    client: reqwest::Client,
}

impl GroqClient {
    /// Create new Groq client loading GROQ_API_KEY from env or ~/.config/keymind/.env.
    pub fn new() -> Result<Self, AiError> {
        Self::load_env();

        let api_key = env::var("GROQ_API_KEY").map_err(|_| AiError::MissingApiKey)?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| AiError::Network(e.to_string()))?;

        Ok(Self { api_key, client })
    }

    /// Loads environment variables from ~/.config/keymind/.env if present.
    pub fn load_env() {
        if let Some(home) = dirs_next::home_dir() {
            let env_path: PathBuf = home.join(".config").join("keymind").join(".env");
            let _ = dotenvy::from_path(env_path);
        }
        let _ = dotenvy::dotenv();
    }
}

#[async_trait]
impl GroqClientTrait for GroqClient {
    async fn generate(&self, system_prompt: &str, user_content: &str) -> Result<String, AiError> {
        let payload = GroqRequest {
            model: "llama-3.3-70b-versatile".to_string(),
            messages: vec![
                GroqMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                GroqMessage {
                    role: "user".to_string(),
                    content: user_content.to_string(),
                },
            ],
            max_tokens: 400,
            temperature: 0.3,
        };

        let mut backoff = 2u64;

        for attempt in 0..3 {
            let res_result = self
                .client
                .post("https://api.groq.com/openai/v1/chat/completions")
                .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
                .header(CONTENT_TYPE, "application/json")
                .json(&payload)
                .send()
                .await;

            match res_result {
                Ok(res) => {
                    if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        info!(
                            "Groq API 429 Rate Limited. Attempt {}. Sleeping for {}s",
                            attempt + 1,
                            backoff
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
                        backoff *= 2;
                        continue;
                    }

                    if res.status().is_success() {
                        let resp_json: GroqResponse = res
                            .json()
                            .await
                            .map_err(|e| AiError::Network(e.to_string()))?;

                        if let Some(choice) = resp_json.choices.first() {
                            return Ok(choice.message.content.trim().to_string());
                        }
                    } else {
                        let err_text = res.text().await.unwrap_or_default();
                        return Err(AiError::ApiError(err_text));
                    }
                }
                Err(e) => {
                    info!("Groq API connection error: {}. Retrying...", e);
                    tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
                    backoff *= 2;
                }
            }
        }

        Err(AiError::ApiError("Failed to get response after retries".to_string()))
    }
}

/// Helper returning default prompts for the 5 built-in AI variables.
pub fn get_ai_system_prompt(key: &str) -> &'static str {
    match key {
        "leave" | "/leave" => {
            "Generate a formal leave application letter.\nRules:\n- Address to: Manager/HR (use \"To Whom It May Concern\" if no specific name in context)\n- If clipboard contains dates or reason: use them. Otherwise: reason = \"personal reasons\", dates = starting tomorrow for 2 days\n- Format: Date, To line, Subject line, 3–4 sentence body, professional sign-off\n- Keep it under 120 words total\nOutput ONLY the letter text. No preamble. Start directly with the date."
        }
        "reply" | "/reply" => {
            "The user message contains an email they received. Write a professional, concise reply.\nRules:\n- Acknowledge the main point or request from the original email\n- Respond appropriately to any questions asked\n- Match the formality level of the original\n- 3–5 sentences maximum\n- Start directly with a salutation (Dear [Name], / Hi [Name],)\nOutput ONLY the reply text. No \"Here is a reply:\" preamble."
        }
        "meeting" | "/meeting" => {
            "Generate a professional meeting agenda.\nRules:\n- If clipboard contains meeting details (topic, attendees, time): use them\n- Otherwise: create a generic team meeting agenda\n- Format (plain text, no markdown):\n    MEETING AGENDA\n    Date: [date]\n    Objective: [one sentence]\n    \n    1. [Item] — [X min]\n    2. [Item] — [X min]\n    3. [Item] — [X min]\n    4. [Item] — [X min]\n    \n    Next Steps: [2–3 action items]\n- No markdown. No bullet characters — use numbers for agenda items.\nOutput ONLY the agenda."
        }
        "summarize" | "/summarize" => {
            "Summarize the provided text into 2–4 bullet points.\nEach bullet: starts with •, one sentence, begins with a strong verb, captures a key insight.\nOutput ONLY the bullets. No preamble. No headers."
        }
        "translate" | "/translate" => {
            "Detect the language of the input text. If it is already English, improve its clarity instead.\nIf it is another language: translate it into natural, fluent English.\nOutput format:\n[Translated or improved text here]\nNo \"Detected:\" prefix. No explanation. Output ONLY the result text."
        }
        _ => "You are a helpful text generation assistant. Output ONLY the requested result.",
    }
}

/// Mock Groq client for unit and integration testing.
pub struct MockGroqClient {
    responses: HashMap<String, String>,
    should_timeout: bool,
}

impl Default for MockGroqClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockGroqClient {
    pub fn new() -> Self {
        let mut responses = HashMap::new();
        responses.insert(
            "leave".to_string(),
            "Dear Manager, Please accept this formal leave application...".to_string(),
        );
        responses.insert(
            "reply".to_string(),
            "Thank you for your message. I am pleased to confirm...".to_string(),
        );
        responses.insert(
            "meeting".to_string(),
            "Meeting Agenda:\n1. Project Overview\n2. Q&A".to_string(),
        );
        responses.insert(
            "summarize".to_string(),
            "• Point 1\n• Point 2\n• Point 3".to_string(),
        );
        responses.insert(
            "translate".to_string(),
            "Translated text in English.".to_string(),
        );

        Self {
            responses,
            should_timeout: false,
        }
    }

    pub fn with_timeout(mut self) -> Self {
        self.should_timeout = true;
        self
    }
}

#[async_trait]
impl GroqClientTrait for MockGroqClient {
    async fn generate(&self, _system_prompt: &str, user_content: &str) -> Result<String, AiError> {
        if self.should_timeout {
            tokio::time::sleep(Duration::from_millis(100)).await;
            return Err(AiError::Timeout);
        }

        // Match based on prompt content or user content
        if user_content.contains("leave") || _system_prompt.contains("leave") {
            Ok(self.responses["leave"].clone())
        } else if user_content.contains("reply") || _system_prompt.contains("reply") {
            Ok(self.responses["reply"].clone())
        } else if user_content.contains("meeting") || _system_prompt.contains("meeting") {
            Ok(self.responses["meeting"].clone())
        } else if user_content.contains("summarize") || _system_prompt.contains("summarize") {
            Ok(self.responses["summarize"].clone())
        } else if user_content.contains("translate") || _system_prompt.contains("translate") {
            Ok(self.responses["translate"].clone())
        } else {
            Ok("Mock generated response".to_string())
        }
    }
}
