// This function receives a number n and returns the nth number in the fibonacci series.
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    
    if n == 1 {
        return 1;
    }

    let mut first: u32 = 0;
    let mut second: u32 = 1;

    for _ in 2..=n {
        let next = first + second;
        first = second;
        second = next;
    }

    second
}

