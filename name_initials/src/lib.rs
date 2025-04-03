// Create a function named initials. 
// This function will receive a vector of string literals with names, 
// and return a vector of Strings with the initials of each name.


pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_vec: Vec<String> = vec![];
    
    for name in names {
        let mut initials = String::new();
        
        
        for part in name.split_whitespace() {
            
            if let Some(first_char) = part.chars().next() {
                initials.push(first_char.to_ascii_uppercase());
                initials.push('.'); 
                initials.push(' ');
            }
        }
        if !initials.is_empty() {
            initials.pop();
        }
        new_vec.push(initials);
    }

    new_vec
}
