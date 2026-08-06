pub mod cache;
pub mod fixer;
pub mod server_process;
pub mod shortcut;

use cache::GrammarCache;
use fixer::apply_text_fixes;
use nlprule::{Rules, Tokenizer};
use serde::{Deserialize, Serialize};
pub use shortcut::SelectionFixer;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarIssue {
    pub offset: usize,
    pub length: usize,
    pub message: String,
    pub replacements: Vec<String>,
    pub rule_id: String,
    pub category: String, // "TYPOS" | "GRAMMAR" | "STYLE" | "PUNCTUATION"
}

static TOKENIZER: OnceLock<Tokenizer> = OnceLock::new();
static RULES: OnceLock<Rules> = OnceLock::new();

fn get_nlprule_engine() -> Option<(&'static Tokenizer, &'static Rules)> {
    let tokenizer = TOKENIZER.get_or_init(|| {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/en_tokenizer.bin"));
        Tokenizer::new(Cursor::new(bytes)).expect("Failed to parse embedded nlprule tokenizer")
    });

    let rules = RULES.get_or_init(|| {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/en_rules.bin"));
        Rules::new(Cursor::new(bytes)).expect("Failed to parse embedded nlprule rules")
    });

    Some((tokenizer, rules))
}

pub struct GrammarEngine {
    cache: GrammarCache,
    is_ready: Arc<AtomicBool>,
}

impl Default for GrammarEngine {
    fn default() -> Self {
        Self::new(8081)
    }
}

impl GrammarEngine {
    /// Construct GrammarEngine (port parameter preserved for backwards API compatibility).
    pub fn new(_port: u16) -> Self {
        Self {
            cache: GrammarCache::default(),
            is_ready: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Legacy constructor preserved for compatibility.
    pub fn with_java_server(_jar_path: PathBuf, _port: u16) -> Self {
        Self::new(_port)
    }

    /// Initialize and warm up the native nlprule engine.
    pub async fn start(&self) {
        let _ = tokio::task::spawn_blocking(|| {
            let _ = get_nlprule_engine();
        })
        .await;

        self.is_ready.store(true, Ordering::SeqCst);
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::SeqCst)
    }

    /// Synchronously or asynchronously check text for grammar issues using native Rust nlprule.
    /// Runs sub-millisecond to ~10ms without any Java process or HTTP overhead.
    pub async fn check_text(&self, text: &str, _language: &str) -> Vec<GrammarIssue> {
        let text_trimmed = text.trim();
        if text_trimmed.is_empty() {
            return Vec::new();
        }

        // 1. Check LRU Cache first
        if let Some(cached) = self.cache.get(text_trimmed) {
            return cached;
        }

        let text_owned = text_trimmed.to_string();
        let issues = tokio::task::spawn_blocking(move || {
            let (tokenizer, rules) = match get_nlprule_engine() {
                Some(e) => e,
                None => return Vec::new(),
            };

            let suggestions = rules.suggest(&text_owned, tokenizer);
            suggestions
                .into_iter()
                .map(|s| {
                    let start_char = text_owned[..s.start()].chars().count();
                    let len_char = text_owned[s.start()..s.end()].chars().count();
                    let rule_id = s.rule_id().to_string();

                    let category = if rule_id.contains("TYPO") || rule_id.contains("SPELL") {
                        "TYPOS".to_string()
                    } else if rule_id.contains("STYLE") {
                        "STYLE".to_string()
                    } else if rule_id.contains("PUNCTUATION") || rule_id.contains("COMMA") {
                        "PUNCTUATION".to_string()
                    } else {
                        "GRAMMAR".to_string()
                    };

                    GrammarIssue {
                        offset: start_char,
                        length: len_char,
                        message: s.message().to_string(),
                        replacements: s
                            .replacements()
                            .iter()
                            .map(|r| r.value().to_string())
                            .collect(),
                        rule_id,
                        category,
                    }
                })
                .collect::<Vec<GrammarIssue>>()
        })
        .await
        .unwrap_or_default();

        // Save to LRU cache
        self.cache.put(text_trimmed, issues.clone());
        issues
    }

    /// Automatically applies top suggestion for each issue found in text.
    pub async fn fix_text(&self, text: &str) -> String {
        let issues = self.check_text(text, "en-US").await;
        apply_text_fixes(text, &issues)
    }
}
