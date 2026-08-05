pub mod cache;
pub mod fixer;
pub mod server_process;
pub mod shortcut;

use cache::GrammarCache;
use fixer::apply_text_fixes;
use serde::{Deserialize, Serialize};
use server_process::ServerProcessManager;
pub use shortcut::SelectionFixer;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarIssue {
    pub offset: usize,
    pub length: usize,
    pub message: String,
    pub replacements: Vec<String>,
    pub rule_id: String,
    pub category: String, // "TYPOS" | "GRAMMAR" | "STYLE" | "PUNCTUATION"
}

#[derive(Debug, Deserialize)]
struct LtReplacement {
    value: String,
}

#[derive(Debug, Deserialize)]
struct LtCategory {
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LtRule {
    id: String,
    category: Option<LtCategory>,
}

#[derive(Debug, Deserialize)]
struct LtMatch {
    offset: usize,
    length: usize,
    message: String,
    replacements: Vec<LtReplacement>,
    rule: LtRule,
}

#[derive(Debug, Deserialize)]
struct LtResponse {
    matches: Vec<LtMatch>,
}

pub struct GrammarEngine {
    client: reqwest::Client,
    process_manager: Option<Arc<ServerProcessManager>>,
    cache: GrammarCache,
    is_ready: Arc<AtomicBool>,
    port: u16,
}

impl GrammarEngine {
    /// Construct GrammarEngine with custom server port.
    pub fn new(port: u16) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_millis(2000))
                .build()
                .unwrap_or_default(),
            process_manager: None,
            cache: GrammarCache::default(),
            is_ready: Arc::new(AtomicBool::new(false)),
            port,
        }
    }

    /// Construct GrammarEngine managing LanguageTool Java process.
    pub fn with_java_server(jar_path: PathBuf, port: u16) -> Self {
        let mgr = Arc::new(ServerProcessManager::new(jar_path, port));
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_millis(2000))
                .build()
                .unwrap_or_default(),
            process_manager: Some(mgr),
            cache: GrammarCache::default(),
            is_ready: Arc::new(AtomicBool::new(false)),
            port,
        }
    }

    /// Start server process (if configured) and perform warmup request.
    pub async fn start(&self) {
        if let Some(ref mgr) = self.process_manager {
            let started = mgr.start_server().await;
            self.is_ready.store(started, Ordering::SeqCst);
        } else {
            // Assume external server is running
            self.is_ready.store(true, Ordering::SeqCst);
        }

        // Perform warm up check
        let _ = self.check_text("Hello world.", "en-US").await;
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::SeqCst)
    }

    /// Asynchronously check text for grammar issues against LanguageTool.
    /// Max timeout: 2000ms. Returns empty Vec on timeout or failure.
    pub async fn check_text(&self, text: &str, language: &str) -> Vec<GrammarIssue> {
        let text_trimmed = text.trim();
        if text_trimmed.is_empty() {
            return Vec::new();
        }

        // 1. Check LRU Cache first
        if let Some(cached) = self.cache.get(text_trimmed) {
            return cached;
        }

        // 2. Perform HTTP request to LanguageTool /v2/check
        let url = format!("http://localhost:{}/v2/check", self.port);
        let params = [
            ("text", text_trimmed),
            ("language", if language.is_empty() { "en-US" } else { language }),
        ];

        let req_future = self.client.post(&url).form(&params).send();

        let resp_result = match timeout(Duration::from_millis(2000), req_future).await {
            Ok(Ok(res)) => res,
            Ok(Err(e)) => {
                warn!("LanguageTool HTTP request failed: {}", e);
                return Vec::new();
            }
            Err(_) => {
                warn!("LanguageTool check_text timed out after 2000ms");
                return Vec::new();
            }
        };

        if !resp_result.status().is_success() {
            return Vec::new();
        }

        let lt_resp: LtResponse = match resp_result.json().await {
            Ok(json) => json,
            Err(e) => {
                warn!("Failed to parse LanguageTool response JSON: {}", e);
                return Vec::new();
            }
        };

        let issues: Vec<GrammarIssue> = lt_resp
            .matches
            .into_iter()
            .map(|m| {
                let cat = m
                    .rule
                    .category
                    .and_then(|c| c.id)
                    .unwrap_or_else(|| "GRAMMAR".to_string());

                GrammarIssue {
                    offset: m.offset,
                    length: m.length,
                    message: m.message,
                    replacements: m.replacements.into_iter().map(|r| r.value).collect(),
                    rule_id: m.rule.id,
                    category: cat,
                }
            })
            .collect();

        // Save to cache
        self.cache.put(text_trimmed, issues.clone());
        issues
    }

    /// Automatically applies top suggestion for each issue found in text.
    pub async fn fix_text(&self, text: &str) -> String {
        let issues = self.check_text(text, "en-US").await;
        apply_text_fixes(text, &issues)
    }
}
