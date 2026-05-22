use ibus_vie_vi::tone::{apply_tone, base_vowel, get_tone, Tone};

/// The preedit buffer that tracks raw keystrokes and produces Vietnamese output.
///
/// This is shared logic used by all engines after they map their specific
/// key sequences to buffer operations.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// Raw keystrokes (the Telex/VNI sequence typed so far).
    raw: Vec<char>,
    /// The composed Vietnamese output from the raw buffer.
    composed: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            raw: Vec::new(),
            composed: String::new(),
        }
    }

    /// Get the current composed (Vietnamese) string.
    pub fn composed(&self) -> &str {
        &self.composed
    }

    /// Get the raw keystrokes.
    pub fn raw(&self) -> &[char] {
        &self.raw
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Clear the buffer entirely.
    pub fn clear(&mut self) {
        self.raw.clear();
        self.composed.clear();
    }

    /// Push a raw character and update composed output.
    /// `transform` is a closure that takes the current composed string and new char,
    /// and returns the new composed string.
    pub fn push(&mut self, c: char, transform: impl FnOnce(&str, char) -> String) {
        self.raw.push(c);
        self.composed = transform(&self.composed, c);
    }

    /// Set the composed string directly (used after complex transformations).
    pub fn set_composed(&mut self, s: String) {
        self.composed = s;
    }

    /// Remove the last operation (backspace behavior in preedit).
    /// Returns true if something was removed, false if buffer was empty.
    pub fn pop(&mut self) -> bool {
        if self.raw.is_empty() {
            return false;
        }
        self.raw.pop();
        true
    }

    /// Take the composed content out, clearing the buffer.
    pub fn take(&mut self) -> String {
        let result = std::mem::take(&mut self.composed);
        self.raw.clear();
        result
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply a tone mark to the correct vowel position in the composed string.
pub fn apply_tone_to_composed(composed: &str, tone: Tone) -> String {
    ibus_vie_vi::tone::place_tone(composed, tone)
}

/// Transform a vowel in the composed string by applying a mark (circumflex, breve, horn).
pub fn apply_mark_to_composed(composed: &str, base: char, target: char) -> String {
    let chars: Vec<char> = composed.chars().collect();

    // Find the last vowel that matches the base and apply mark
    let mut result = chars.clone();
    for i in (0..chars.len()).rev() {
        let lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
        let b = base_vowel(lower);
        if b == base {
            let is_upper = chars[i].is_uppercase();
            // Preserve any existing tone
            let existing_tone = get_tone(chars[i]);
            let marked = if is_upper {
                target.to_uppercase().next().unwrap_or(target)
            } else {
                target
            };
            result[i] = apply_tone(marked, existing_tone);
            break;
        }
    }
    result.into_iter().collect()
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
        let mut buf = Buffer::new();
        assert!(buf.is_empty());
        buf.push('a', |_, c| c.to_string());
        assert_eq!(buf.composed(), "a");
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_buffer_take() {
        let mut buf = Buffer::new();
        buf.push('h', |s, c| format!("{}{}", s, c));
        buf.push('a', |s, c| format!("{}{}", s, c));
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
