use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static font_cc_regex: Regex =
        Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap();
    static font_otp_regex: Regex = Regex::new(r"^\d{4,8}$").unwrap();
}

pub struct PrivacyFilter {
    blocklist: HashSet<String>,
}

impl Default for PrivacyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivacyFilter {
    pub fn new() -> Self {
        Self {
            blocklist: HashSet::new(),
        }
    }

    pub fn set_blocklist(&mut self, list: HashSet<String>) {
        self.blocklist = list;
    }

    pub fn add_to_blocklist(&mut self, app_id: &str) {
        self.blocklist.insert(app_id.to_string());
    }

    /// Returns true if text/event passes all privacy checks (is safe to store).
    pub fn is_safe(&self, text: &str, app_id: Option<&str>, is_sensitive: bool) -> bool {
        // 1. Sensitive field flag check
        if is_sensitive {
            return false;
        }

        // 2. App blocklist check
        if let Some(app) = app_id {
            if self.blocklist.contains(app) {
                return false;
            }
        }

        // 3. Credit Card regex check
        if font_cc_regex.is_match(text) {
            return false;
        }

        // 4. Standalone OTP / PIN regex check
        for token in text.split_whitespace() {
            let clean_token = token.trim_matches(|c: char| !c.is_alphanumeric());
            if font_otp_regex.is_match(clean_token) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_filter_credit_card() {
        let filter = PrivacyFilter::new();
        assert!(!filter.is_safe("4111 2222 3333 4444", None, false));
        assert!(!filter.is_safe("Card 4111-2222-3333-4444 test", None, false));
        assert!(filter.is_safe("Meeting tomorrow at 4pm", None, false));
    }

    #[test]
    fn test_privacy_filter_otp_pin() {
        let filter = PrivacyFilter::new();
        assert!(!filter.is_safe("Your code is 849201", None, false));
        assert!(filter.is_safe("Project 2026 update", None, false));
    }

    #[test]
    fn test_privacy_filter_sensitive_flag() {
        let filter = PrivacyFilter::new();
        assert!(!filter.is_safe("normal text", None, true));
    }
}
