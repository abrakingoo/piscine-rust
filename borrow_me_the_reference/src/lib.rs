// delete_and_backspace: which receives a borrowed string, and processes it. 
// - represents the backspace key and + represents the delete key, 
// so that "helll-o" and "he+lllo" are both converted to "hello". 
// The - and + characters should be removed from the string.
pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    
    // First pass: Handle '+' (delete operation)
    while i < chars.len() {
        if chars[i] == '+' {
            chars.remove(i);
            if i < chars.len() {
                chars.remove(i);
            }
        } else {
            i += 1;
        }
    }

    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i);
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
}




// borrows a vector of string literals representing simple addition and subtraction equations. 
// The function should replace the operation with the result
pub fn do_operations(v: &mut [String]) {
    for item in v.iter_mut() {

        // Parsing the values and handling addition
        if item.contains('+') {
            let values: Vec<&str> = item.split('+').collect();

            if let (Ok(val1), Ok(val2)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                let res = val1 + val2;
                *item = res.to_string();
            }
        }

        // Parsing the values and handling subtraction
        if item.contains('-') {
            let values: Vec<&str> = item.split('-').collect();

            if let (Ok(val1), Ok(val2)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                let res = val1 - val2;
                *item = res.to_string();
            }
        }
    }
}
