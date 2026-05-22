use crate::buffer::Buffer;
use crate::engine::Engine;
use vi::processor::AccentStyle;

/// Telex input method engine.
///
/// Delegates all Vietnamese text transformation to the `vi` crate.
#[derive(Debug, Clone)]
pub struct TelexEngine {
    buffer: Buffer,
}

impl TelexEngine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(&vi::TELEX, AccentStyle::Old),
        }
    }
}

impl Default for TelexEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for TelexEngine {
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}
