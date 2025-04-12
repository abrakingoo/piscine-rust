// ✅ How it works:
// For each character:

// If it's an ASCII letter, determine its base (A or a) and compute the rotated position.

// Use .rem_euclid(26) instead of % to handle negative keys correctly.

// If it's not a letter (punctuation, number, etc.), it remains unchanged.

// Collect all the mapped characters into a new String.

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = (c as u8 - base) as i8;
                let rotated = (offset + key).rem_euclid(26) as u8;
                (base + rotated) as char
            } else {
                c
            }
        })
        .collect()
}
