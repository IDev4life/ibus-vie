use crate::action::{Action, KeyEvent};
use crate::buffer::{apply_mark_to_composed, apply_tone_to_composed, is_commit_trigger, Buffer};
use crate::engine::Engine;
use ibus_vie_vi::alphabet::is_vowel;
use ibus_vie_vi::tone::Tone;

/// Telex input method engine.
#[derive(Debug, Clone)]
pub struct TelexEngine {
    buffer: Buffer,
}

impl TelexEngine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    /// Check if a character is a Telex tone key.
    fn tone_key(c: char) -> Option<Tone> {
        match c.to_lowercase().next().unwrap_or(c) {
            's' => Some(Tone::Acute),
            'f' => Some(Tone::Grave),
            'r' => Some(Tone::Hook),
            'x' => Some(Tone::Tilde),
            'j' => Some(Tone::Dot),
            'z' => Some(Tone::None), // Remove tone
            _ => None,
        }
    }

    /// Check if a character is a Telex mark key that modifies a vowel.
    /// Returns (base_vowel_to_find, resulting_marked_vowel).
    fn mark_key(c: char, composed: &str) -> Option<MarkAction> {
        let lower = c.to_lowercase().next().unwrap_or(c);
        match lower {
            'w' => {
                // 'w' can produce: ă (from a), ơ (from o), ư (from u)
                let chars: Vec<char> = composed.chars().collect();
                for i in (0..chars.len()).rev() {
                    let ch = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                    let base = ibus_vie_vi::tone::base_vowel(ch);
                    match base {
                        'u' => return Some(MarkAction { base: 'u', target: 'ư' }),
                        'o' => return Some(MarkAction { base: 'o', target: 'ơ' }),
                        'a' => return Some(MarkAction { base: 'a', target: 'ă' }),
                        _ => continue,
                    }
                }
                None
            }
            'a' => {
                // 'aa' -> â
                if composed.chars().last().map(|ch| {
                    let b = ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                    b == 'a'
                }).unwrap_or(false) {
                    Some(MarkAction { base: 'a', target: 'â' })
                } else {
                    None
                }
            }
            'e' => {
                // 'ee' -> ê
                if composed.chars().last().map(|ch| {
                    let b = ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                    b == 'e'
                }).unwrap_or(false) {
                    Some(MarkAction { base: 'e', target: 'ê' })
                } else {
                    None
                }
            }
            'o' => {
                // 'oo' -> ô
                if composed.chars().last().map(|ch| {
                    let b = ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                    b == 'o'
                }).unwrap_or(false) {
                    Some(MarkAction { base: 'o', target: 'ô' })
                } else {
                    None
                }
            }
            'd' => {
                // 'dd' -> đ
                // Check if composed ends with 'd' or 'D'
                if composed.chars().last().map(|ch| ch.to_lowercase().next().unwrap_or(ch) == 'd').unwrap_or(false) {
                    // Check it's actually a 'd' not 'đ'
                    let last = composed.chars().last().unwrap();
                    if last == 'd' || last == 'D' {
                        return Some(MarkAction { base: 'd', target: 'đ' });
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Check if the composed string contains any vowel.
    fn has_vowel(composed: &str) -> bool {
        composed.chars().any(is_vowel)
    }

    /// Process a regular character (not a tone/mark key).
    fn process_regular(&mut self, c: char) -> Action {
        self.buffer.push(c, |s, ch| format!("{}{}", s, ch));
        Action::Update
    }
}

struct MarkAction {
    base: char,
    target: char,
}

impl Default for TelexEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for TelexEngine {
    fn key(&mut self, ev: KeyEvent) -> Action {
        // Handle backspace
        if ev.backspace {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            // Recompute from raw keystrokes minus the last one
            let raw: Vec<char> = self.buffer.raw().to_vec();
            self.buffer.clear();
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

        // Check if this is a commit trigger (space, punctuation, etc.)
        if is_commit_trigger(c) {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Check for 'dd' -> đ
        if (c == 'd' || c == 'D')
            && Self::mark_key(c, self.buffer.composed()).is_some_and(|m| m.base == 'd')
        {
            // Replace 'd' with 'đ'
            let composed = self.buffer.composed().to_string();
            let new_composed = replace_last_char(&composed, c);
            self.buffer.push(c, |_, _| new_composed);
            return Action::Update;
        }

        // If buffer has vowels, check for tone keys
        if Self::has_vowel(self.buffer.composed()) {
            if let Some(tone) = Self::tone_key(c) {
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_tone_to_composed(&composed, tone);
                self.buffer.push(c, |_, _| new_composed);
                return Action::Update;
            }
        }

        // Check for mark keys (aa->â, ee->ê, oo->ô, aw->ă, ow->ơ, uw->ư)
        if !self.buffer.is_empty() {
            if let Some(mark) = Self::mark_key(c, self.buffer.composed()) {
                if mark.base == 'd' {
                    // dd -> đ: replace last 'd' with 'đ'
                    let composed = self.buffer.composed().to_string();
                    let new_composed = replace_last_char(&composed, if c.is_uppercase() { 'Đ' } else { 'đ' });
                    self.buffer.push(c, |_, _| new_composed);
                    return Action::Update;
                }
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_mark_to_composed(&composed, mark.base, mark.target);
                self.buffer.push(c, |_, _| new_composed);
                return Action::Update;
            }
        }

        // If character is not alphabetic and buffer is not empty, commit buffer
        if !c.is_alphabetic() {
            if self.buffer.is_empty() {
                return Action::PassThrough;
            }
            let mut committed = self.buffer.take();
            committed.push(c);
            return Action::Commit(committed);
        }

        // Regular character: just append
        self.process_regular(c)
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn preedit(&self) -> &str {
        self.buffer.composed()
    }
}

/// Replace the last character in a string that matches `old` (case-insensitive match to 'd')
/// with the Vietnamese đ/Đ.
fn replace_last_char(s: &str, _replacement: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    for i in (0..chars.len()).rev() {
        if chars[i] == 'd' || chars[i] == 'D' {
            chars[i] = if chars[i].is_uppercase() { 'Đ' } else { 'đ' };
            break;
        }
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(input: &str) -> String {
        let mut engine = TelexEngine::new();
        engine.feed_str(input)
    }

    #[test]
    fn test_basic_vowel() {
        assert_eq!(feed("a"), "a");
        assert_eq!(feed("ba"), "ba");
    }

    #[test]
    fn test_tone_acute() {
        // "as" → 'a' then 's' (tone key) → "á"
        assert_eq!(feed("as"), "á");
    }

    #[test]
    fn test_viet() {
        // vieetj -> "việt"
        // v -> v, i -> vi, e -> vie, e -> viê (ee->ê), t -> viêt, j -> việt (dot tone)
        assert_eq!(feed("vieetj"), "việt");
    }

    #[test]
    fn test_tieng() {
        // tieengs -> "tiếng"
        assert_eq!(feed("tieengs"), "tiếng");
    }

    #[test]
    fn test_dd() {
        // ddi -> đi
        assert_eq!(feed("ddi"), "đi");
    }

    #[test]
    fn test_chao() {
        assert_eq!(feed("chaof"), "chào");
    }

    #[test]
    fn test_nguoi() {
        // nguwowif -> người
        assert_eq!(feed("nguwowif"), "người");
    }

    #[test]
    fn test_space_commit() {
        let mut engine = TelexEngine::new();
        let result = engine.feed_str("xin chao");
        assert_eq!(result, "xin chao");
    }

    #[test]
    fn test_ddau() {
        assert_eq!(feed("ddaau"), "đâu");
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(feed("DDi"), "Đi");
    }
}
