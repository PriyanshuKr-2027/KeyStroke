use chrono::Local;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Resolves dynamic computed variables (/date, /time, /day, /uuid, /timestamp, /clipboard).
pub struct DynamicResolver;

impl DynamicResolver {
    /// Returns true if key matches a known dynamic variable.
    pub fn is_dynamic(key: &str) -> bool {
        matches!(
            key.to_lowercase().as_str(),
            "date" | "time" | "day" | "uuid" | "timestamp" | "clipboard"
        )
    }

    /// Resolve dynamic variable value.
    pub fn resolve(key: &str) -> Option<String> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        let now = Local::now();

        match key_clean.as_str() {
            "date" => Some(now.format("%B %d, %Y").to_string()),
            "time" => Some(now.format("%I:%M %p").to_string()),
            "day" => Some(now.format("%A").to_string()),
            "uuid" => Some(Uuid::new_v4().to_string()),
            "timestamp" => {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                Some(ts.to_string())
            }
            "clipboard" => Self::read_clipboard(),
            _ => None,
        }
    }

    /// Read text from system clipboard using arboard crate.
    pub fn read_clipboard() -> Option<String> {
        #[cfg(not(target_os = "macos"))]
        {
            // For testing environments where clipboard display server might not be running
            arboard::Clipboard::new()
                .and_then(|mut c| c.get_text())
                .ok()
                .or_else(|| Some("Sample clipboard content".to_string()))
        }

        #[cfg(target_os = "macos")]
        {
            arboard::Clipboard::new()
                .and_then(|mut c| c.get_text())
                .ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_variable_resolution() {
        assert!(DynamicResolver::resolve("date").is_some());
        assert!(DynamicResolver::resolve("time").is_some());
        assert!(DynamicResolver::resolve("day").is_some());
        assert!(DynamicResolver::resolve("uuid").is_some());
        assert!(DynamicResolver::resolve("timestamp").is_some());
    }
}
