pub fn abbreviate(phrase: &str) -> String {
    let (acronym, _, _) = phrase.chars().fold(
        (String::new(), false, false),
        |(mut acronym, in_word, prev_uppercase), ch| {
            if ch.is_ascii_alphabetic() {
                if !in_word || (ch.is_ascii_uppercase() && !prev_uppercase) {
                    acronym.push(ch.to_ascii_uppercase());
                }
                (acronym, true, ch.is_ascii_uppercase())
            } else if ch.is_whitespace() || ch == '-' {
                (acronym, false, false)
            } else {
                (acronym, in_word, prev_uppercase)
            }
        },
    );

    acronym
}
