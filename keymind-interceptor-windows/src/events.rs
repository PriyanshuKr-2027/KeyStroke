use std::fmt;

/// Keyboard modifiers snapshot for Windows key events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub win: bool,
}

/// Events emitted by the Windows keyboard interceptor engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    PermissionRequired,
    KeyPress {
        char: Option<char>,
        vk_code: u32,
        modifiers: Modifiers,
    },
    SensitiveFieldKeyPress,
    WordCompleted {
        word: String,
        context: String,
    },
    EngineError(&'static str),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::PermissionRequired => write!(f, "Event::PermissionRequired"),
            Event::KeyPress { char, vk_code, modifiers } => {
                write!(f, "Event::KeyPress(char={:?}, vk={}, mods={:?})", char, vk_code, modifiers)
            }
            Event::SensitiveFieldKeyPress => write!(f, "Event::SensitiveFieldKeyPress"),
            Event::WordCompleted { word, context } => {
                write!(f, "Event::WordCompleted(word=\"{}\", context=\"{}\")", word, context)
            }
            Event::EngineError(err) => write!(f, "Event::EngineError(\"{}\")", err),
        }
    }
}
