
fn main() {
    use std::io;
    let mut input = String::new();
    let answer = "e";
    let mut count = 0;

    loop {
        print!("I am the beginning of the end, and the end of time and space. \nI am essential to creation, and I surround every place. What am I?: ");
        
        // Clear the input buffer before each new read
        input.clear();
        
        // Read the user input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Trim the input to remove any trailing newline characters
        let input = input.trim();

        // Check if the answer is correct
        match input {
            // If input matches answer, break the loop
            s if s == answer => {
                println!("Number of trials: {}", count);
                break;
            }
            // Otherwise, keep asking for input
            _ => {
                count += 1;
                continue;
            }
        }
    }

    // Output the number of attempts (incorrect answers before the correct one)
    println!("Number of trials: {}", count);
}

