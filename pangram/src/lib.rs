use std::collections::HashSet;

// ✅ Explanation:
// We use a HashSet to keep track of unique alphabetic characters (ignoring case).

// c.is_ascii_alphabetic() ensures only letters are considered.

// to_ascii_lowercase() makes the check case-insensitive.

// A pangram must contain all 26 letters of the alphabet — so we check that letters.len() == 26.

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new();

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            letters.insert(c.to_ascii_lowercase());
        }
    }

    letters.len() == 26
}
