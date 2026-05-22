use vi::methods::IncrementalBuffer;
use vi::processor::AccentStyle;
use vi::Definition;

/// The preedit buffer that tracks raw keystrokes and produces Vietnamese output.
///
/// Wraps vi-rs `IncrementalBuffer` which handles all tone placement,
/// letter modification, and undo logic internally.
#[derive(Debug, Clone)]
pub struct Buffer {
    inner: IncrementalBuffer<'static>,
}

impl Buffer {
    /// Create a new buffer with the given typing definition and accent style.
    pub fn new(definition: &'static Definition, style: AccentStyle) -> Self {
        Self {
            inner: IncrementalBuffer::new_with_style(definition, style),
        }
    }

    /// Get the current composed (Vietnamese) string.
    pub fn composed(&self) -> &str {
        self.inner.view()
    }

    /// Get the raw keystrokes.
    pub fn raw(&self) -> &[char] {
        self.inner.input()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear the buffer entirely.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Push a character and let vi-rs handle the transformation.
    pub fn push(&mut self, c: char) {
        self.inner.push(c);
    }

    /// Take the composed content out, clearing the buffer.
    pub fn take(&mut self) -> String {
        let result = self.inner.view().to_string();
        self.inner.clear();
        result
    }
}

/// Check if a character triggers a word boundary (commit point).
pub fn is_commit_trigger(c: char) -> bool {
    matches!(
        c,
        ' ' | '\n'
            | '\t'
            | ','
            | '.'
            | ';'
            | ':'
            | '!'
            | '?'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '"'
            | '\''
            | '/'
            | '\\'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_basic() {
        let mut buf = Buffer::new(&vi::TELEX, AccentStyle::Old);
        assert!(buf.is_empty());
        buf.push('a');
        assert_eq!(buf.composed(), "a");
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_buffer_take() {
        let mut buf = Buffer::new(&vi::TELEX, AccentStyle::Old);
        buf.push('h');
        buf.push('a');
        assert_eq!(buf.take(), "ha");
        assert!(buf.is_empty());
    }

    #[test]
    fn test_is_commit_trigger() {
        assert!(is_commit_trigger(' '));
        assert!(is_commit_trigger('.'));
        assert!(!is_commit_trigger('a'));
        assert!(!is_commit_trigger('1'));
    }
}
