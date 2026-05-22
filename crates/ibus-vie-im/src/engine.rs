use crate::action::{Action, KeyEvent};

/// Trait for Vietnamese input method engines.
///
/// Each engine (Telex, VNI, VIQR) implements this trait.
/// The engine is pure logic — no I/O, no IBus dependency.
pub trait Engine {
    /// Push a key event into the engine. Returns an action describing what to do.
    fn key(&mut self, ev: KeyEvent) -> Action;

    /// Reset the preedit buffer (e.g. on focus change, Escape).
    fn reset(&mut self);

    /// Get the current preedit string for display.
    fn preedit(&self) -> &str;

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
