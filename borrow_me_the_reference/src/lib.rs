// delete_and_backspace: which receives a borrowed string, and processes it. 
// - represents the backspace key and + represents the delete key, 
// so that "helll-o" and "he+lllo" are both converted to "hello". 
// The - and + characters should be removed from the string.
pub fn delete_and_backspace(s: &mut String) {
    let mut new_string = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                new_string.pop();
            }
            '+' => {
                if let Some(_) = chars.next() {
                    // Handle delete: remove the next character by consuming it
                }
            }
            _ => {
                new_string.push(c); 
            }
        }
    }

    *s = new_string.iter().collect();
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
