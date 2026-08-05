use std::fmt;

/// Keyboard modifiers snapshot for key events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub option: bool,
    pub command: bool,
}

impl Modifiers {
    pub fn is_empty(&self) -> bool {
        !self.shift && !self.control && !self.option && !self.command
    }
}

/// Events emitted by the keyboard interceptor engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Emitted when macOS Accessibility permission is not granted.
    PermissionRequired,

    /// Emitted on normal non-sensitive keydown event.
    KeyPress {
        key: char,
        modifiers: Modifiers,
    },

    /// Emitted on keypress in secure / password fields. Key content is masked/omitted.
    SensitiveFieldKeyPress,

    /// Emitted on word boundary character (space, comma, period, enter, tab).
    WordCompleted {
        word: String,
        context: String,
    },

    /// Engine failure alert (e.g. "tap_dead").
    EngineError(&'static str),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::PermissionRequired => write!(f, "Event::PermissionRequired"),
            Event::KeyPress { key, modifiers } => {
                write!(f, "Event::KeyPress(key='{}', mods={:?})", key, modifiers)
            }
            Event::SensitiveFieldKeyPress => write!(f, "Event::SensitiveFieldKeyPress"),
            Event::WordCompleted { word, context } => {
                write!(f, "Event::WordCompleted(word=\"{}\", context=\"{}\")", word, context)
            }
            Event::EngineError(err) => write!(f, "Event::EngineError(\"{}\")", err),
        }
    }
}
