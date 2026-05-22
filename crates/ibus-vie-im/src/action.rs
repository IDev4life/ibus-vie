/// A key event received from the input system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The character produced by this key press (if printable).
    pub char: Option<char>,
    /// Whether this is a backspace key.
    pub backspace: bool,
    /// Whether this is an escape key.
    pub escape: bool,
}

impl KeyEvent {
    /// Create a key event from a character.
    pub fn from_char(c: char) -> Self {
        Self {
            char: Some(c),
            backspace: false,
            escape: false,
        }
    }

    /// Create a backspace key event.
    pub fn backspace() -> Self {
        Self {
            char: None,
            backspace: true,
            escape: false,
        }
    }

    /// Create an escape key event.
    pub fn escape() -> Self {
        Self {
            char: None,
            backspace: false,
            escape: true,
        }
    }
}

/// The action the engine wants the IBus glue to perform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Update the preedit display (underlined text).
    Update,
    /// Commit the given string to the application and clear preedit.
    Commit(String),
    /// Engine does not handle this key; pass it through to the application.
    PassThrough,
}
