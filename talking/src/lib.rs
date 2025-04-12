// 🔍 Breakdown:
// trim() is used to ignore spaces when checking if the input is empty.

// is_question is true if the string ends with a ?.

// has_letters ensures that yelling is only detected if there are alphabetic characters.

// is_yelling checks if all letters are uppercase (no lowercase letters at all).

// We match combinations of yelling/question flags to return the proper response.

pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_question = trimmed.ends_with('?');
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && trimmed.chars().all(|c| !c.is_lowercase());

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
