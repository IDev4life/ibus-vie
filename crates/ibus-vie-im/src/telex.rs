use crate::action::Action;
use crate::buffer::{apply_mark_to_composed, apply_tone_to_composed, Buffer};
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
                // 'w' can produce: ư (from u), ơ (from o), ă (from a)
                // Priority: check vowel pairs "uo"→"ươ" and "ua"→"ưa" first
                let chars: Vec<char> = composed.chars().collect();

                // Look for "uo" or "ua" pair (u followed by o/a in vowel cluster)
                for i in 0..chars.len().saturating_sub(1) {
                    let base_i = ibus_vie_vi::tone::base_vowel(
                        chars[i].to_lowercase().next().unwrap_or(chars[i]),
                    );
                    if base_i == 'u' {
                        let base_next = ibus_vie_vi::tone::base_vowel(
                            chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]),
                        );
                        if base_next == 'o' {
                            // "uo" pair → mark both: u→ư, o→ơ
                            return Some(MarkAction {
                                base: 'u',
                                target: 'ư',
                                also_mark: Some(('o', 'ơ')),
                            });
                        } else if base_next == 'a' {
                            // "ua" pair → mark u→ư only
                            return Some(MarkAction {
                                base: 'u',
                                target: 'ư',
                                also_mark: None,
                            });
                        }
                    }
                }

                // Fallback: scan right-to-left for single vowel
                for i in (0..chars.len()).rev() {
                    let ch = chars[i].to_lowercase().next().unwrap_or(chars[i]);
                    let base = ibus_vie_vi::tone::base_vowel(ch);
                    match base {
                        'u' => {
                            return Some(MarkAction {
                                base: 'u',
                                target: 'ư',
                                also_mark: None,
                            })
                        }
                        'o' => {
                            return Some(MarkAction {
                                base: 'o',
                                target: 'ơ',
                                also_mark: None,
                            })
                        }
                        'a' => {
                            return Some(MarkAction {
                                base: 'a',
                                target: 'ă',
                                also_mark: None,
                            })
                        }
                        _ => continue,
                    }
                }
                None
            }
            'a' => {
                // 'aa' -> â
                if composed
                    .chars()
                    .last()
                    .map(|ch| {
                        let b =
                            ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                        b == 'a'
                    })
                    .unwrap_or(false)
                {
                    Some(MarkAction {
                        base: 'a',
                        target: 'â',
                        also_mark: None,
                    })
                } else {
                    None
                }
            }
            'e' => {
                // 'ee' -> ê
                if composed
                    .chars()
                    .last()
                    .map(|ch| {
                        let b =
                            ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                        b == 'e'
                    })
                    .unwrap_or(false)
                {
                    Some(MarkAction {
                        base: 'e',
                        target: 'ê',
                        also_mark: None,
                    })
                } else {
                    None
                }
            }
            'o' => {
                // 'oo' -> ô
                if composed
                    .chars()
                    .last()
                    .map(|ch| {
                        let b =
                            ibus_vie_vi::tone::base_vowel(ch.to_lowercase().next().unwrap_or(ch));
                        b == 'o'
                    })
                    .unwrap_or(false)
                {
                    Some(MarkAction {
                        base: 'o',
                        target: 'ô',
                        also_mark: None,
                    })
                } else {
                    None
                }
            }
            'd' => {
                // 'dd' -> đ
                // Check if composed ends with 'd' or 'D'
                if composed
                    .chars()
                    .last()
                    .map(|ch| ch.to_lowercase().next().unwrap_or(ch) == 'd')
                    .unwrap_or(false)
                {
                    // Check it's actually a 'd' not 'đ'
                    let last = composed.chars().last().unwrap();
                    if last == 'd' || last == 'D' {
                        return Some(MarkAction {
                            base: 'd',
                            target: 'đ',
                            also_mark: None,
                        });
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
}

struct MarkAction {
    base: char,
    target: char,
    /// Optional second mark to apply (e.g., "uo" → "ươ" needs both u→ư and o→ơ).
    also_mark: Option<(char, char)>,
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

    fn process_char(&mut self, c: char) -> Option<Action> {
        // Check for 'dd' -> đ
        if (c == 'd' || c == 'D')
            && Self::mark_key(c, self.buffer.composed()).is_some_and(|m| m.base == 'd')
        {
            let composed = self.buffer.composed().to_string();
            let new_composed = replace_last_char(&composed, c);
            self.buffer.push(c, |_, _| new_composed);
            return Some(Action::Update);
        }

        // If buffer has vowels, check for tone keys
        if Self::has_vowel(self.buffer.composed()) {
            if let Some(tone) = Self::tone_key(c) {
                let composed = self.buffer.composed().to_string();
                let new_composed = apply_tone_to_composed(&composed, tone);
                self.buffer.push(c, |_, _| new_composed);
                return Some(Action::Update);
            }
        }

        // Check for mark keys (aa->â, ee->ê, oo->ô, uw->ư, uo+w->ươ, ua+w->ưa)
        if !self.buffer.is_empty() {
            if let Some(mark) = Self::mark_key(c, self.buffer.composed()) {
                if mark.base == 'd' {
                    let composed = self.buffer.composed().to_string();
                    let new_composed =
                        replace_last_char(&composed, if c.is_uppercase() { 'Đ' } else { 'đ' });
                    self.buffer.push(c, |_, _| new_composed);
                    return Some(Action::Update);
                }
                let composed = self.buffer.composed().to_string();
                let mut new_composed = apply_mark_to_composed(&composed, mark.base, mark.target);
                if let Some((also_base, also_target)) = mark.also_mark {
                    new_composed = apply_mark_to_composed(&new_composed, also_base, also_target);
                }
                self.buffer.push(c, |_, _| new_composed);
                return Some(Action::Update);
            }
        }

        None
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
