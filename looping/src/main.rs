use std::io::{self, Write};

fn main() {

    let mut input = String::new();
    let answer = "e";
    let mut count = 0;

    loop {

                print!("I am the beginning of the end, and the end of time and space. 
I am essential to creation, and I surround every place. What am I?: ");
            
                io::stdout().flush().unwrap();
            
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
            
                let input = input.trim();
            
                if input != answer {
                    count += 1;
                    println!("Your Answer: {}",input);
                } else {
                    break;
                }

        }

        println!("{}", count)
}
