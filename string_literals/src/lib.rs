// Create the following functions:

// is_empty: that returns true if the string is empty.
// is_ascii: that returns true if all characters are within the ASCII range.
// contains: that returns true if the string contains the given pattern.
// split_at: that divides a string in two returning a tuple.
// find: that returns the index of the first character of a given string that matches the pattern.

pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    if let Some(i) = v.find(pat) {
        return i;
    }
    return 0
}