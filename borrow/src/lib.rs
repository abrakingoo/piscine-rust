// Create a function named str_len, you'll need to complete the function signature. 
// Your function should accept a borrowed string, and return its length (in characters).

pub fn str_len(s: &str ) -> usize {
    s.chars().count()
}