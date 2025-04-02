// a function named rev_str that takes a &str as a parameter, and returns a String with its letters reversed.
pub fn rev_str(input: &str) -> String {
    let mut new_string = String::new();
    for ch in input.chars().rev() {
        new_string.push(ch);
    }
    new_string
}
