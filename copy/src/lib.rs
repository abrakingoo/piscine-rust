// Create the following functions. The objective is to know how ownership works with different types.

// nbr_function returns a tuple:
        // with the original value.
        // the exponential function of the value.
        // and the natural logarithm of the absolute value.
pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exponential_value = (c as f64).exp();
    let logarithm_value = (c.abs() as f64).ln();

    (original_value, exponential_value, logarithm_value)
}

// str_function returns a tuple:
        // with the original value.
        // and the exponential function of each value as a string (see the example).
pub fn str_function(a: String) -> (String, String) {
    let original_value = a.clone();

    // Create a string to hold the result of the exponentiation of each character
    let mut exponential_values = String::new();

    // Iterate over each character in the string
    for c in a.chars() {
        // Convert the character to its ASCII value (using `as u8`), then calculate the exponential
        let exp_val = (f64::exp(c as u8 as f64)).to_string();
        
        // Append the result to the string
        exponential_values.push_str(&exp_val);
        exponential_values.push(' '); // Space between each exponential value for readability
    }

    // Return the tuple: (original string, string with exponential values)
    (original_value, exponential_values.trim().to_string()) // trim to remove the last extra space
}

// vec_function returns a tuple:
        // with the original value.
        // and the natural logarithm of each absolute value.
pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let original_value = b.clone();

    // Create a vector to store the logarithms of the absolute values
    let log_values: Vec<f64> = b.iter()
        .map(|&x| f64::ln(x.abs() as f64))  // Compute ln of the absolute value of each element
        .collect();

    // Return the tuple: (original vector, vector of log values)
    (original_value, log_values)
}