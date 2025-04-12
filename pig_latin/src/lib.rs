pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    let mut chars = text.chars();

    // Check if the first letter is a vowel
    if let Some(first_char) = chars.clone().next() {
        if vowels.contains(first_char) {
            return format!("{}ay", text); // If the word starts with a vowel, add "ay"
        }
    }

    // Check for "qu" as a consonant cluster
    if text.starts_with("qu") {
        return format!("{}ay", &text[2..] + "qu"); // Move "qu" to the end
    }

    // Otherwise, find the first vowel and apply the transformation
    for (i, c) in text.chars().enumerate() {
        if vowels.contains(c) {
            // Move all consonants before the first vowel to the end
            return format!("{}{}ay", &text[i..], &text[..i]);
        }
    }

    text.to_string() // In case of an empty or non-vowel starting word
}

// Explanation:
// First Check: If the first character is a vowel, we just append "ay" to the word.

// Second Check: If the word starts with "qu", it gets moved to the end of the word and "ay" is appended.

// Default Case: We loop through the string, find the first vowel, and then move all consonants before it to the end.