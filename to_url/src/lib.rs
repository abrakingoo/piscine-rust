pub fn to_url(s: &str) -> String {
    let mut new_str = String::new();
    for x in s.chars() {
        if x == ' ' {
            new_str.push_str("%20");
        } else {
            new_str.push(x)
        }
    }
    new_str
}