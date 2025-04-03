pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words by the number in each word
    words.sort_by(|a, b| {
        let a_number = a.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();
        let b_number = b.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();
        a_number.cmp(&b_number)
    });

    // Remove numbers and return the sorted words as a single string
    let sorted_words: Vec<String> = words.iter()
        .map(|&word| word.chars().filter(|c| !c.is_digit(10)).collect())
        .collect();

    sorted_words.join(" ")
}
