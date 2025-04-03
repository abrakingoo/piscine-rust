// Create the following functions. The objective is to know how ownership works with different types.

// nbr_function returns a tuple:
        // with the original value.
        // the exponential function of the value.
        // and the natural logarithm of the absolute value.
pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original_value = c.clone();
    let exponential_value = (c as f64).exp();
    let logarithm_value = (c.abs() as f64).ln();

    (original_value, exponential_value, logarithm_value)
}

// str_function returns a tuple:
        // with the original value.
        // and the exponential function of each value as a string (see the example).
pub fn str_function(a: String) -> (String, String) {
            // Split the string into individual parts
            let parts: Vec<&str> = a.split_whitespace().collect();
        
            // Calculate the exponential function for each part and collect into a new Vec
            let exp_values: Vec<String> = parts
                .iter()
                .map(|&part| {
                    let value: f64 = part.parse().unwrap(); // Parse the string into a float
                    format!("{}", value.exp()) // Calculate the exponential and convert to string
                })
                .collect();
        
            // Join the calculated exponential values into a single string
            let exp_string = exp_values.join(" ");
        
            // Return the original string and the exponential string as a tuple
            (a, exp_string)
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