/// Actions returned by the trigger detection state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerAction {
    /// Continue buffering characters.
    Capturing,
    /// Trigger boundary reached with a potential variable key.
    Expand {
        key: String,
        backspace_count: usize,
    },
    /// Buffer cleared/cancelled due to invalid character or overflow.
    Cancelled,
}

/// State machine tracking trigger start (`/`) and variable key buffering.
#[derive(Debug, Default)]
pub struct TriggerDetector {
    is_capturing: bool,
    buffer: String,
    max_len: usize,
}

impl TriggerDetector {
    pub fn new() -> Self {
        Self {
            is_capturing: false,
            buffer: String::new(),
            max_len: 20,
        }
    }

    /// Process a single keystroke character.
    pub fn process_char(&mut self, c: char) -> TriggerAction {
        if !self.is_capturing {
            if c == '/' {
                self.is_capturing = true;
                self.buffer.clear();
                return TriggerAction::Capturing;
            }
            return TriggerAction::Cancelled;
        }

        // Boundary character (space, enter, tab)
        if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
            if !self.buffer.is_empty() {
                let key = std::mem::take(&mut self.buffer);
                let backspace_count = 1 + key.len(); // 1 for '/' + key length
                self.is_capturing = false;
                return TriggerAction::Expand {
                    key,
                    backspace_count,
                };
            } else {
                self.reset();
                return TriggerAction::Cancelled;
            }
        }

        // Alphanumeric character
        if c.is_alphanumeric() || c == '_' || c == '-' {
            if self.buffer.len() >= self.max_len {
                self.reset();
                return TriggerAction::Cancelled;
            }
            self.buffer.push(c);
            return TriggerAction::Capturing;
        }

        // Non-alphanumeric character immediately cancels
        self.reset();
        TriggerAction::Cancelled
    }

    /// Reset trigger state machine.
    pub fn reset(&mut self) {
        self.is_capturing = false;
        self.buffer.clear();
    }

    pub fn is_capturing(&self) -> bool {
        self.is_capturing
    }

    pub fn current_buffer(&self) -> &str {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_detection_success() {
        let mut detector = TriggerDetector::new();
        assert_eq!(detector.process_char('/'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('d'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('a'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('t'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('e'), TriggerAction::Capturing);

        let action = detector.process_char(' ');
        assert_eq!(
            action,
            TriggerAction::Expand {
                key: "date".to_string(),
                backspace_count: 5, // '/' + 4 chars
            }
        );
    }

    #[test]
    fn test_trigger_detection_cancellation_non_alpha() {
        let mut detector = TriggerDetector::new();
        assert_eq!(detector.process_char('/'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('a'), TriggerAction::Capturing);
        assert_eq!(detector.process_char('!'), TriggerAction::Cancelled);
        assert!(!detector.is_capturing());
    }

    #[test]
    fn test_trigger_detection_cancellation_overflow() {
        let mut detector = TriggerDetector::new();
        detector.process_char('/');
        for _ in 0..21 {
            detector.process_char('a');
        }
        assert!(!detector.is_capturing());
    }
}
