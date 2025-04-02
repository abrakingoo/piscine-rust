// inserts a new element at the end of the Vec.
pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

// returns the value found at the index passed as an argument.
pub fn at_index(slice: &[String], index: usize) -> &str {
    let res = &slice[index];
    res.to_string();
}
