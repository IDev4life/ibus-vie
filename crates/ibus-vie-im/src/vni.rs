use crate::action::Action;
use crate::buffer::{apply_mark_to_composed, apply_tone_to_composed, Buffer};
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
                if composed
                    .chars()
                    .last()
                    .map(|ch| ch == 'd' || ch == 'D')
                    .unwrap_or(false)
                {
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
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn process_char(&mut self, c: char) -> Option<Action> {
        // VNI uses digit keys for marks and tones
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
                    return Some(Action::Update);
                }
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_mark_to_composed(&composed, base, target);
                self.buffer.push(c, |_, _| new_composed);
                return Some(Action::Update);
            }

            // Check tone keys (1-5, 0) when there are vowels
            if Self::has_vowel(self.buffer.composed()) {
                if let Some(tone) = Self::tone_key(c) {
                    let composed = self.buffer.composed().to_string();
                    let new_composed = apply_tone_to_composed(&composed, tone);
                    self.buffer.push(c, |_, _| new_composed);
                    return Some(Action::Update);
                }
            }

            // Digit not consumed — let default handling commit or pass through
            return None;
        }

        None
    }
}
