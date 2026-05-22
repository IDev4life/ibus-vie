/// Vietnamese tones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// No tone mark (thanh ngang / thanh không).
    None,
    /// Sắc (acute accent).
    Acute,
    /// Huyền (grave accent).
    Grave,
    /// Hỏi (hook above).
    Hook,
    /// Ngã (tilde).
    Tilde,
    /// Nặng (dot below).
    Dot,
}

/// Map of base vowels to their toned variants.
/// Order: [None, Acute, Grave, Hook, Tilde, Dot]
const TONE_TABLE: &[(char, [char; 6])] = &[
    ('a', ['a', 'á', 'à', 'ả', 'ã', 'ạ']),
    ('ă', ['ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ']),
    ('â', ['â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ']),
    ('e', ['e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ']),
    ('ê', ['ê', 'ế', 'ề', 'ể', 'ễ', 'ệ']),
    ('i', ['i', 'í', 'ì', 'ỉ', 'ĩ', 'ị']),
    ('o', ['o', 'ó', 'ò', 'ỏ', 'õ', 'ọ']),
    ('ô', ['ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ']),
    ('ơ', ['ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ']),
    ('u', ['u', 'ú', 'ù', 'ủ', 'ũ', 'ụ']),
    ('ư', ['ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự']),
    ('y', ['y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ']),
];

/// Get the base vowel (without tone) for a given character.
/// Returns the character itself if it's not a toned vowel.
pub fn base_vowel(c: char) -> char {
    let lower = c.to_lowercase().next().unwrap_or(c);
    for &(base, ref variants) in TONE_TABLE {
        if variants.contains(&lower) || lower == base {
            return base;
        }
    }
    c
}

/// Get the current tone on a character.
pub fn get_tone(c: char) -> Tone {
    let lower = c.to_lowercase().next().unwrap_or(c);
    for &(_base, ref variants) in TONE_TABLE {
        for (i, &v) in variants.iter().enumerate() {
            if v == lower {
                return match i {
                    0 => Tone::None,
                    1 => Tone::Acute,
                    2 => Tone::Grave,
                    3 => Tone::Hook,
                    4 => Tone::Tilde,
                    5 => Tone::Dot,
                    _ => Tone::None,
                };
            }
        }
    }
    Tone::None
}

/// Apply a tone to a vowel character. Preserves case.
pub fn apply_tone(c: char, tone: Tone) -> char {
    let is_upper = c.is_uppercase();
    let lower = c.to_lowercase().next().unwrap_or(c);
    let base = base_vowel(lower);

    let tone_idx = match tone {
        Tone::None => 0,
        Tone::Acute => 1,
        Tone::Grave => 2,
        Tone::Hook => 3,
        Tone::Tilde => 4,
        Tone::Dot => 5,
    };

    let result = TONE_TABLE
        .iter()
        .find(|(b, _)| *b == base)
        .map(|(_, variants)| variants[tone_idx])
        .unwrap_or(c);

    if is_upper {
        result.to_uppercase().next().unwrap_or(result)
    } else {
        result
    }
}

/// Place tone on the correct vowel in a syllable string.
/// Uses the "new style" rule (kiểu mới).
///
/// Algorithm:
/// 1. Single vowel → tone on it
/// 2. If exactly one vowel has a diacritic mark (ê, ô, ơ, ư, â, ă) → tone on it
/// 3. 3+ vowels → tone on the middle one (e.g., "ươi" → ơ)
/// 4. 2 vowels with coda consonant → tone on the second (last) vowel
/// 5. 2 vowels without coda → tone on the first vowel
pub fn place_tone(syllable: &str, tone: Tone) -> String {
    if tone == Tone::None {
        // Remove existing tones
        return syllable
            .chars()
            .map(|c| apply_tone(c, Tone::None))
            .collect();
    }

    let chars: Vec<char> = syllable.chars().collect();
    let vowel_positions: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| crate::alphabet::is_vowel(**c))
        .map(|(i, _)| i)
        .collect();

    if vowel_positions.is_empty() {
        return syllable.to_string();
    }

    let target = if vowel_positions.len() == 1 {
        // Rule 1: Single vowel
        vowel_positions[0]
    } else {
        // Rule 2: Check for a single marked vowel (ê, ô, ơ, ư, â, ă)
        let marked_positions: Vec<usize> = vowel_positions
            .iter()
            .copied()
            .filter(|&pos| {
                let base = base_vowel(chars[pos].to_lowercase().next().unwrap_or(chars[pos]));
                matches!(base, 'ê' | 'ô' | 'ơ' | 'ư' | 'â' | 'ă')
            })
            .collect();

        if marked_positions.len() == 1 {
            // Exactly one marked vowel: tone goes on it
            marked_positions[0]
        } else if vowel_positions.len() >= 3 {
            // Rule 3: 3+ vowels → middle one
            vowel_positions[vowel_positions.len() / 2]
        } else {
            // Rules 4 & 5: 2 vowels
            let last_vowel_pos = *vowel_positions.last().unwrap();
            let has_coda = if last_vowel_pos + 1 < chars.len() {
                chars[last_vowel_pos + 1..]
                    .iter()
                    .any(|c| crate::alphabet::is_consonant(*c))
            } else {
                false
            };

            if has_coda {
                // Rule 4: With coda → second (last) vowel
                vowel_positions[1]
            } else {
                // Rule 5: Without coda → first vowel
                vowel_positions[0]
            }
        }
    };

    // Remove old tones and apply new tone at target
    let mut result: Vec<char> = chars.iter().map(|c| apply_tone(*c, Tone::None)).collect();
    result[target] = apply_tone(result[target], tone);
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_tone() {
        assert_eq!(apply_tone('a', Tone::Acute), 'á');
        assert_eq!(apply_tone('ê', Tone::Grave), 'ề');
        assert_eq!(apply_tone('ơ', Tone::Dot), 'ợ');
        assert_eq!(apply_tone('A', Tone::Tilde), 'Ã');
    }

    #[test]
    fn test_base_vowel() {
        assert_eq!(base_vowel('á'), 'a');
        assert_eq!(base_vowel('ề'), 'ê');
        assert_eq!(base_vowel('ự'), 'ư');
    }

    #[test]
    fn test_get_tone() {
        assert_eq!(get_tone('á'), Tone::Acute);
        assert_eq!(get_tone('ề'), Tone::Grave);
        assert_eq!(get_tone('a'), Tone::None);
    }

    #[test]
    fn test_place_tone_single_vowel() {
        assert_eq!(place_tone("ba", Tone::Acute), "bá");
        assert_eq!(place_tone("me", Tone::Grave), "mè");
    }

    #[test]
    fn test_place_tone_diphthong_with_coda() {
        // "hoang" -> tone on 'a' (second vowel, has coda 'ng')
        assert_eq!(place_tone("hoang", Tone::Grave), "hoàng");
    }

    #[test]
    fn test_place_tone_diphthong_open() {
        // New style: 2 vowels no coda -> first vowel: "hoa" -> "hòa"
        assert_eq!(place_tone("hoa", Tone::Grave), "hòa");
    }

    #[test]
    fn test_place_tone_ai_ending() {
        // "ai" diphthong, no coda -> tone on first vowel 'a'
        assert_eq!(place_tone("hai", Tone::Grave), "hài");
    }

    #[test]
    fn test_place_tone_ao_ending() {
        // "ao" diphthong, no coda -> tone on first vowel 'a'
        assert_eq!(place_tone("chao", Tone::Grave), "chào");
    }

    #[test]
    fn test_place_tone_three_vowels() {
        // 3 vowels "ươi" -> tone on middle 'ơ'
        assert_eq!(place_tone("ngươi", Tone::Grave), "người");
    }
}
