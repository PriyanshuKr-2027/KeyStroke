use crate::GrammarIssue;

pub fn apply_text_fixes(text: &str, issues: &[GrammarIssue]) -> String {
    if issues.is_empty() {
        return text.to_string();
    }

    // Sort issues by offset descending to apply replacements right-to-left
    let mut sorted_issues = issues.to_vec();
    sorted_issues.sort_by(|a, b| b.offset.cmp(&a.offset));

    let mut result = text.to_string();

    for issue in sorted_issues {
        if let Some(replacement) = issue.replacements.first() {
            let start = issue.offset;
            let end = start + issue.length;

            if start <= result.len() && end <= result.len() {
                // Ensure character boundary safety
                if result.is_char_boundary(start) && result.is_char_boundary(end) {
                    result.replace_range(start..end, replacement);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_text_fixes_right_to_left() {
        let text = "He are going to teh store.";
        let issues = vec![
            GrammarIssue {
                offset: 3,
                length: 3,
                message: "Did you mean 'is'?".to_string(),
                replacements: vec!["is".to_string()],
                rule_id: "HE_ARE".to_string(),
                category: "GRAMMAR".to_string(),
            },
            GrammarIssue {
                offset: 16,
                length: 3,
                message: "Did you mean 'the'?".to_string(),
                replacements: vec!["the".to_string()],
                rule_id: "TEH_TYPO".to_string(),
                category: "TYPOS".to_string(),
            },
        ];

        let fixed = apply_text_fixes(text, &issues);
        assert_eq!(fixed, "He is going to the store.");
    }
}
