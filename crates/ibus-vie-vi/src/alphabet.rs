/// Vietnamese vowels (base forms without diacritics).
pub const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];

/// Vietnamese vowels with diacritics (breve, circumflex, horn).
pub const VOWELS_WITH_MARKS: &[char] = &['ă', 'â', 'ê', 'ô', 'ơ', 'ư'];

/// All vowel characters (base + marked).
pub const ALL_VOWELS: &[char] = &[
    'a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y',
];

/// Single-character consonants.
pub const SINGLE_CONSONANTS: &[&str] = &[
    "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
];

/// Multi-character onset consonants (phụ âm đầu).
pub const ONSET_CLUSTERS: &[&str] = &[
    "ch", "gh", "gi", "kh", "ng", "ngh", "nh", "ph", "qu", "th", "tr",
];

/// Valid coda consonants (phụ âm cuối).
pub const CODA_CONSONANTS: &[&str] = &[
    "c", "ch", "m", "n", "ng", "nh", "p", "t",
];

/// Check if a character is a Vietnamese vowel (case-insensitive).
pub fn is_vowel(c: char) -> bool {
    let lower = c.to_lowercase().next().unwrap_or(c);
    ALL_VOWELS.contains(&lower)
}

/// Check if a character is a consonant letter (case-insensitive).
pub fn is_consonant(c: char) -> bool {
    let lower = c.to_lowercase().next().unwrap_or(c);
    matches!(lower, 'b'..='d' | 'đ' | 'g' | 'h' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'x')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('A'));
        assert!(is_vowel('ư'));
        assert!(!is_vowel('b'));
        assert!(!is_vowel('đ'));
    }

    #[test]
    fn test_is_consonant() {
        assert!(is_consonant('b'));
        assert!(is_consonant('đ'));
        assert!(!is_consonant('a'));
        assert!(!is_consonant('ê'));
    }
}
