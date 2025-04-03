pub fn first_subword(s: String) -> String {

    //handles snake_case
    if s.contains("_") {
        let parts: Vec<&str> = s.split("_").collect();
        return parts[0].to_string();
    }


    //handles CamelCase
    if !s.is_empty() && s.chars().next().unwrap().is_uppercase() {
        let mut words_vector = Vec::new();
        let mut word = String::new();

        for c in s.chars() {
            if c.is_uppercase() && !word.is_empty() {
                words_vector.push(word);
                word = String::new();
            }

            word.push(c);
        }

        if !word.is_empty() {
            words_vector.push(word);
        }

        return words_vector[0].clone();
    };

    // handles pascalCase
        let mut word = String::new();
        for c in s.chars() {
            if c.is_uppercase() && !word.is_empty() {
                return word.to_string();
            }

            word.push(c);
        }

        s
}