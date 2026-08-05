use std::collections::VecDeque;

/// Rolling context buffer capped at 100 characters.
#[derive(Debug, Clone)]
pub struct ContextBuffer {
    buffer: VecDeque<char>,
    current_word: String,
    max_capacity: usize,
}

impl Default for ContextBuffer {
    fn default() -> Self {
        Self::new(100)
    }
}

impl ContextBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            current_word: String::new(),
            max_capacity: capacity,
        }
    }

    /// Check if a character represents a word boundary.
    pub fn is_word_boundary(c: char) -> bool {
        matches!(c, ' ' | ',' | '.' | '\n' | '\r' | '\t')
    }

    /// Push a non-sensitive key press character into the buffer.
    /// Returns `Some((word, context))` if a word boundary was reached and a non-empty word was completed.
    pub fn push_char(&mut self, c: char) -> Option<(String, String)> {
        // Maintain rolling character buffer limit
        if self.buffer.len() >= self.max_capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);

        if Self::is_word_boundary(c) {
            if !self.current_word.is_empty() {
                let word = std::mem::take(&mut self.current_word);
                let context = self.get_context();
                return Some((word, context));
            }
        } else {
            self.current_word.push(c);
        }

        None
    }

    /// Returns the current context buffer as a String.
    pub fn get_context(&self) -> String {
        self.buffer.iter().collect()
    }

    /// Reset/clear current word state without clearing rolling buffer.
    pub fn clear_word(&mut self) {
        self.current_word.clear();
    }

    /// Clear all buffer and word state.
    pub fn clear_all(&mut self) {
        self.buffer.clear();
        self.current_word.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_buffer_rolling_limit() {
        let mut ctx = ContextBuffer::new(5);
        for c in "abcdefgh" .chars() {
            ctx.push_char(c);
        }
        assert_eq!(ctx.get_context(), "defgh");
    }

    #[test]
    fn test_word_completion() {
        let mut ctx = ContextBuffer::new(100);
        let mut completed = None;
        for c in "hello ".chars() {
            if let Some(res) = ctx.push_char(c) {
                completed = Some(res);
            }
        }
        let (word, context) = completed.expect("Should trigger word completion");
        assert_eq!(word, "hello");
        assert_eq!(context, "hello ");
    }
}
