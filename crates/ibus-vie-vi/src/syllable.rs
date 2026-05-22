use crate::alphabet;
use crate::tone::Tone;

/// Represents a parsed Vietnamese syllable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllable {
    /// Onset consonant(s), e.g. "tr", "ng", "kh", or empty.
    pub onset: String,
    /// Nucleus (vowel cluster), e.g. "ươ", "oa", "a".
    pub nucleus: String,
    /// Coda consonant(s), e.g. "ng", "ch", "t", or empty.
    pub coda: String,
    /// Tone applied to the syllable.
    pub tone: Tone,
}

impl Syllable {
    /// Check if a string could be the start of a valid Vietnamese syllable.
    /// Used to decide whether to keep characters in the preedit buffer.
    pub fn is_valid_prefix(s: &str) -> bool {
        if s.is_empty() {
            return true;
        }

        let lower = s.to_lowercase();
        let chars: Vec<char> = lower.chars().collect();

        // Check if it starts with a valid onset
        let mut pos = 0;

        // Try to match onset consonants
        if pos < chars.len() && alphabet::is_consonant(chars[pos]) {
            // Try multi-char onsets first (longest match)
            let remaining: String = chars[pos..].iter().collect();
            let mut matched_onset = false;
            // Sort by length descending to match "ngh" before "ng"
            let mut clusters: Vec<&str> = alphabet::ONSET_CLUSTERS.to_vec();
            clusters.sort_by_key(|b| std::cmp::Reverse(b.len()));
            for onset in &clusters {
                if remaining.starts_with(onset) {
                    pos += onset.len();
                    matched_onset = true;
                    break;
                }
            }
            if !matched_onset {
                // Single consonant onset
                pos += 1;
            }
        }

        // After onset, we expect vowels (or end of string = still valid prefix)
        if pos >= chars.len() {
            return true;
        }

        // Check vowels
        let mut has_vowel = false;
        while pos < chars.len() && alphabet::is_vowel(chars[pos]) {
            has_vowel = true;
            pos += 1;
        }

        // If we consumed everything after onset, it's a valid prefix
        if pos >= chars.len() {
            return true;
        }

        // After vowels, check for valid coda
        if has_vowel {
            let remaining: String = chars[pos..].iter().collect();
            for coda in alphabet::CODA_CONSONANTS {
                if *coda == remaining || coda.starts_with(&remaining) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_prefix() {
        assert!(Syllable::is_valid_prefix(""));
        assert!(Syllable::is_valid_prefix("t"));
        assert!(Syllable::is_valid_prefix("tr"));
        assert!(Syllable::is_valid_prefix("tra"));
        assert!(Syllable::is_valid_prefix("tran"));
        assert!(Syllable::is_valid_prefix("trang"));
        assert!(Syllable::is_valid_prefix("ngh"));
        assert!(Syllable::is_valid_prefix("nghi"));
    }

    #[test]
    fn test_invalid_prefix() {
        // Two vowels followed by invalid consonant cluster
        assert!(!Syllable::is_valid_prefix("aab"));
    }
}
