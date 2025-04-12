// 🔍 Explanation:
// Convert the number to a string to extract each digit.

// Count how many digits there are (power).

// Use .map() to raise each digit to that power.

// Sum the results and compare it to the original number.

pub fn number_logic(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let power = digits.len() as u32;

    let sum: u32 = digits.iter().map(|d| d.pow(power)).sum();

    sum == num
}
