use std::collections::VecDeque;

/// Rolling 100-character context buffer for Windows interceptor.
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

    pub fn is_word_boundary(c: char) -> bool {
        matches!(c, ' ' | ',' | '.' | '\n' | '\r' | '\t')
    }

    pub fn push_char(&mut self, c: char) -> Option<(String, String)> {
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

    pub fn get_context(&self) -> String {
        self.buffer.iter().collect()
    }

    pub fn clear_word(&mut self) {
        self.current_word.clear();
    }
}
