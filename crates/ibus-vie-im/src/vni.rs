use crate::action::Action;
use crate::buffer::Buffer;
use crate::engine::Engine;
use vi::processor::AccentStyle;

/// VNI input method engine.
///
/// Delegates all Vietnamese text transformation to the `vi` crate.
/// Overrides `process_char` to intercept digit keys (which vi-rs uses
/// for tones/marks) before the default handler would commit on non-alpha.
#[derive(Debug, Clone)]
pub struct VniEngine {
    buffer: Buffer,
}

impl VniEngine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(&vi::VNI, AccentStyle::Old),
        }
    }
}

impl Default for VniEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for VniEngine {
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn process_char(&mut self, c: char) -> Option<Action> {
        // VNI uses digit keys (0-9) for tones and marks.
        // Intercept them here so the default handler doesn't commit on non-alpha.
        if c.is_ascii_digit() && !self.buffer.is_empty() {
            self.buffer.push(c);
            return Some(Action::Update);
        }
        None
    }
}
