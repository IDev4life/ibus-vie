use crate::action::{Action, KeyEvent};
use crate::buffer::{apply_mark_to_composed, apply_tone_to_composed, Buffer};
use crate::engine::Engine;
use ibus_vie_vi::alphabet::is_vowel;
use ibus_vie_vi::tone::Tone;

/// VIQR input method engine.
/// Uses ASCII punctuation for tone marks and diacritics.
#[derive(Debug, Clone)]
pub struct ViqrEngine {
    buffer: Buffer,
}

impl ViqrEngine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    /// Check if a character is a VIQR tone key.
    fn tone_key(c: char) -> Option<Tone> {
        match c {
            '\'' => Some(Tone::Acute),
            '`' => Some(Tone::Grave),
            '?' => Some(Tone::Hook),
            '~' => Some(Tone::Tilde),
            '.' => Some(Tone::Dot),
            _ => None,
        }
    }

    /// Check if a character is a VIQR mark key.
    fn mark_key(c: char, composed: &str) -> Option<(char, char)> {
        match c {
            '^' => {
                // a^ -> â, e^ -> ê, o^ -> ô
                let chars: Vec<char> = composed.chars().collect();
                for i in (0..chars.len()).rev() {
                    let ch = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                    let base = ibus_vie_vi::tone::base_vowel(ch);
                    match base {
                        'a' => return Some(('a', 'â')),
                        'e' => return Some(('e', 'ê')),
                        'o' => return Some(('o', 'ô')),
                        _ => continue,
                    }
                }
                None
            }
            '(' => {
                // a( -> ă
                let chars: Vec<char> = composed.chars().collect();
                for i in (0..chars.len()).rev() {
                    let ch = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                    let base = ibus_vie_vi::tone::base_vowel(ch);
                    if base == 'a' {
                        return Some(('a', 'ă'));
                    }
                }
                None
            }
            '+' => {
                // o+ -> ơ, u+ -> ư
                let chars: Vec<char> = composed.chars().collect();
                for i in (0..chars.len()).rev() {
                    let ch = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                    let base = ibus_vie_vi::tone::base_vowel(ch);
                    match base {
                        'o' => return Some(('o', 'ơ')),
                        'u' => return Some(('u', 'ư')),
                        _ => continue,
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Check for dd -> đ pattern.
    fn check_dd(c: char, composed: &str) -> bool {
        (c == 'd' || c == 'D')
            && composed
                .chars()
                .last()
                .map(|ch| ch == 'd' || ch == 'D')
                .unwrap_or(false)
    }

    fn has_vowel(composed: &str) -> bool {
        composed.chars().any(is_vowel)
    }
}

impl Default for ViqrEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for ViqrEngine {
    fn key(&mut self, ev: KeyEvent) -> Action {
        if ev.backspace {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let raw: Vec<char> = self.buffer.raw().to_vec();
            self.buffer.clear();
            if raw.len() <= 1 {
                return Action::Update;
            }
            for &c in &raw[..raw.len() - 1] {
                let _ = self.key(KeyEvent::from_char(c));
            }
            return Action::Update;
        }

        if ev.escape {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            self.buffer.clear();
            return Action::Update;
        }

        let c = match ev.char {
            Some(c) => c,
            None => return Action::PassThrough,
        };

        // Space and newline are commit triggers but not VIQR-special
        if c == ' ' || c == '\n' || c == '\t' {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Check for dd -> đ
        if (c == 'd' || c == 'D') && Self::check_dd(c, self.buffer.composed()) {
            let composed = self.buffer.composed().to_string();
            let mut chars: Vec<char> = composed.chars().collect();
            if let Some(last) = chars.last_mut() {
                if *last == 'd' {
                    *last = 'đ';
                } else if *last == 'D' {
                    *last = 'Đ';
                }
            }
            let new_composed: String = chars.into_iter().collect();
            self.buffer.push(c, |_, _| new_composed);
            return Action::Update;
        }

        // Check for mark keys (^, (, +)
        if !self.buffer.is_empty() {
            if let Some((base, target)) = Self::mark_key(c, self.buffer.composed()) {
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_mark_to_composed(&composed, base, target);
                self.buffer.push(c, |_, _| new_composed);
                return Action::Update;
            }
        }

        // Check for tone keys (', `, ?, ~, .)
        // These are special: they double as punctuation, so only apply if buffer has vowels
        if !self.buffer.is_empty() && Self::has_vowel(self.buffer.composed()) {
            if let Some(tone) = Self::tone_key(c) {
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_tone_to_composed(&composed, tone);
                self.buffer.push(c, |_, _| new_composed);
                return Action::Update;
            }
        }

        // Non-alphabetic and not a recognized VIQR key: commit buffer
        if !c.is_alphabetic() {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Regular character
        self.buffer.push(c, |s, ch| format!("{}{}", s, ch));
        Action::Update
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn preedit(&self) -> &str {
        self.buffer.composed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(input: &str) -> String {
        let mut engine = ViqrEngine::new();
        engine.feed_str(input)
    }

    #[test]
    fn test_basic() {
        assert_eq!(feed("a"), "a");
    }

    #[test]
    fn test_circumflex() {
        assert_eq!(feed("a^"), "â");
        assert_eq!(feed("e^"), "ê");
        assert_eq!(feed("o^"), "ô");
    }

    #[test]
    fn test_breve() {
        assert_eq!(feed("a("), "ă");
    }

    #[test]
    fn test_horn() {
        assert_eq!(feed("o+"), "ơ");
        assert_eq!(feed("u+"), "ư");
    }

    #[test]
    fn test_dd() {
        assert_eq!(feed("dd"), "đ");
    }

    #[test]
    fn test_tone() {
        assert_eq!(feed("a'"), "á");
        assert_eq!(feed("a`"), "à");
        assert_eq!(feed("a?"), "ả");
        assert_eq!(feed("a~"), "ã");
        assert_eq!(feed("a."), "ạ");
    }

    #[test]
    fn test_viet_nam() {
        // Vie^.t -> Việt
        assert_eq!(feed("Vie^.t"), "Việt");
    }

    #[test]
    fn test_dau() {
        assert_eq!(feed("dda^u"), "đâu");
    }
}
