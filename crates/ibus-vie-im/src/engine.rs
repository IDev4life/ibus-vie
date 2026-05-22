use crate::action::{Action, KeyEvent};
use crate::buffer::{is_commit_trigger, Buffer};

/// Trait for Vietnamese input method engines.
///
/// Each engine (Telex, VNI) implements this trait.
/// The engine is pure logic — no I/O, no IBus dependency.
///
/// Engines only need to implement `buffer()`, `buffer_mut()`, and optionally
/// `process_char()` for method-specific key interception (e.g., VNI digits).
/// The common key handling is provided by the default `key()` implementation.
pub trait Engine {
    /// Access the internal buffer (immutable).
    fn buffer(&self) -> &Buffer;

    /// Access the internal buffer (mutable).
    fn buffer_mut(&mut self) -> &mut Buffer;

    /// Engine-specific character processing.
    /// Return `Some(Action)` if the engine handled the character specially,
    /// or `None` to fall through to default handling (push to vi-rs buffer).
    fn process_char(&mut self, _c: char) -> Option<Action> {
        None
    }

    /// Push a key event into the engine. Returns an action describing what to do.
    ///
    /// Default implementation handles common boilerplate:
    /// backspace (replay), escape (clear), commit triggers, non-alphabetic commit,
    /// and regular character pushing to vi-rs.
    fn key(&mut self, ev: KeyEvent) -> Action {
        // Handle backspace
        if ev.backspace {
            if self.buffer().is_empty() {
                return Action::PassThrough;
            }
            let raw: Vec<char> = self.buffer().raw().to_vec();
            self.buffer_mut().clear();
            if raw.len() <= 1 {
                return Action::Update;
            }
            // Replay all but last
            for &c in &raw[..raw.len() - 1] {
                let _ = self.key(KeyEvent::from_char(c));
            }
            return Action::Update;
        }

        // Handle escape
        if ev.escape {
            if self.buffer().is_empty() {
                return Action::PassThrough;
            }
            self.buffer_mut().clear();
            return Action::Update;
        }

        let c = match ev.char {
            Some(c) => c,
            None => return Action::PassThrough,
        };

        // Check if this is a commit trigger (space, punctuation, etc.)
        if is_commit_trigger(c) {
            if self.buffer().is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer_mut().take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Engine-specific logic (e.g., VNI digit handling)
        if let Some(action) = self.process_char(c) {
            return action;
        }

        // If character is not alphabetic and buffer is not empty, commit buffer
        if !c.is_alphabetic() {
            if self.buffer().is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer_mut().take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Regular character: push to vi-rs buffer which handles transformations
        self.buffer_mut().push(c);
        Action::Update
    }

    /// Reset the preedit buffer (e.g. on focus change, Escape).
    fn reset(&mut self) {
        self.buffer_mut().clear();
    }

    /// Get the current preedit string for display.
    fn preedit(&self) -> &str {
        self.buffer().composed()
    }

    /// Feed a string of characters and return the final committed + remaining preedit.
    /// Useful for testing.
    fn feed_str(&mut self, input: &str) -> String {
        let mut committed = String::new();
        for c in input.chars() {
            let ev = KeyEvent::from_char(c);
            match self.key(ev) {
                Action::Commit(s) => committed.push_str(&s),
                Action::Update => {}
                Action::PassThrough => {
                    committed.push(c);
                }
            }
        }
        // Append remaining preedit
        let remaining = self.preedit().to_string();
        committed.push_str(&remaining);
        self.reset();
        committed
    }
}
