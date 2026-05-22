use crate::action::{Action, KeyEvent};
use crate::buffer::{apply_mark_to_composed, apply_tone_to_composed, is_commit_trigger, Buffer};
use crate::engine::Engine;
use ibus_vie_vi::alphabet::is_vowel;
use ibus_vie_vi::tone::Tone;

/// VNI input method engine.
/// Uses number keys for tone marks and diacritics.
#[derive(Debug, Clone)]
pub struct VniEngine {
    buffer: Buffer,
}

impl VniEngine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    /// Check if a digit is a VNI tone key.
    fn tone_key(c: char) -> Option<Tone> {
        match c {
            '1' => Some(Tone::Acute),
            '2' => Some(Tone::Grave),
            '3' => Some(Tone::Hook),
            '4' => Some(Tone::Tilde),
            '5' => Some(Tone::Dot),
            '0' => Some(Tone::None),
            _ => None,
        }
    }

    /// Check if a digit is a VNI mark key.
    /// Returns (base_vowel, target_marked_vowel) if applicable.
    fn mark_key(c: char, composed: &str) -> Option<(char, char)> {
        match c {
            '6' => {
                // a6->â, e6->ê, o6->ô
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
            '7' => {
                // o7->ơ, u7->ư
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
            '8' => {
                // a8->ă
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
            '9' => {
                // d9->đ (special: consonant, not vowel)
                if composed.chars().last().map(|ch| ch == 'd' || ch == 'D').unwrap_or(false) {
                    return Some(('d', 'đ'));
                }
                None
            }
            _ => None,
        }
    }

    fn has_vowel(composed: &str) -> bool {
        composed.chars().any(is_vowel)
    }
}

impl Default for VniEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for VniEngine {
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

        if is_commit_trigger(c) {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Check for VNI digit keys when buffer has content
        if c.is_ascii_digit() && !self.buffer.is_empty() {
            // Check mark keys first (6, 7, 8, 9)
            if let Some((base, target)) = Self::mark_key(c, self.buffer.composed()) {
                if base == 'd' {
                    // d9 -> đ
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
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_mark_to_composed(&composed, base, target);
                self.buffer.push(c, |_, _| new_composed);
                return Action::Update;
            }

            // Check tone keys (1-5, 0) when there are vowels
            if Self::has_vowel(self.buffer.composed()) {
                if let Some(tone) = Self::tone_key(c) {
                    let composed = self.buffer.composed().to_string();
                    let new_composed = apply_tone_to_composed(&composed, tone);
                    self.buffer.push(c, |_, _| new_composed);
                    return Action::Update;
                }
            }

            // Digit not consumed: commit buffer + digit
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

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
        let mut engine = VniEngine::new();
        engine.feed_str(input)
    }

    #[test]
    fn test_basic() {
        assert_eq!(feed("a"), "a");
        assert_eq!(feed("ba"), "ba");
    }

    #[test]
    fn test_tone() {
        assert_eq!(feed("a1"), "á");
        assert_eq!(feed("a2"), "à");
    }

    #[test]
    fn test_circumflex() {
        assert_eq!(feed("a6"), "â");
        assert_eq!(feed("e6"), "ê");
    }

    #[test]
    fn test_horn() {
        assert_eq!(feed("o7"), "ơ");
        assert_eq!(feed("u7"), "ư");
    }

    #[test]
    fn test_breve() {
        assert_eq!(feed("a8"), "ă");
    }

    #[test]
    fn test_d_stroke() {
        assert_eq!(feed("d9"), "đ");
    }

    #[test]
    fn test_viet() {
        // vie6t5 -> việt
        assert_eq!(feed("vie6t5"), "việt");
    }

    #[test]
    fn test_dau() {
        assert_eq!(feed("d9a6u"), "đâu");
    }
}
