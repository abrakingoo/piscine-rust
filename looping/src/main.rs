use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let answer = "e";
    let mut count = 0;

    loop {
        print!("I am the beginning of the end, and the end of time and space. \nI am essential to creation, and I surround every place. What am I?: ");
        
        // Flush to ensure the prompt appears before waiting for input
        io::stdout().flush().unwrap();
        
        // Clear the input buffer before each new read
        input.clear();
        
        // Read the user input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Trim the input to remove any trailing newline characters
        let input = input.trim();

        // Check if the answer is correct
        if input == answer {
            break;  // Break out of the loop if the answer is correct
        } else {
            count += 1;  // Increment count if the answer is incorrect
        }
    }

    // Output the number of attempts (incorrect answers before the correct one)
    println!("Number of trials: {}", count);
}

