pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut in_word = false;
    let mut prev_uppercase = false;

    for c in phrase.chars() {
        if c.is_ascii_alphabetic() {
            if !in_word || (c.is_ascii_uppercase() && !prev_uppercase) {
                acronym.push(c.to_ascii_uppercase());
            }
            in_word = true;
            prev_uppercase = c.is_ascii_uppercase();
        } else if c.is_whitespace() || c == '-' {
            in_word = false;
            prev_uppercase = false;
        }
    }

    acronym
}
