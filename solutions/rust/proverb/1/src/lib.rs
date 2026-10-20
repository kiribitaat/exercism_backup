/// Build a proverb from a list of words.
///
/// # Examples
///
/// ```rust
/// use proverb::build_proverb;
///
/// let words = ["nail", "shoe"];
/// let proverb = build_proverb(&words);
/// assert_eq!(proverb, "For want of a nail the shoe was lost.\nAnd all for the want of a nail.");
/// ```
pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        [first, ..] => list
            .windows(2)
            .map(proverb_line)
            .chain(std::iter::once(proverb_end(first)))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

/// Build a proverb line from a window of words.
fn proverb_line(window: &[&str]) -> String {
    format!("For want of a {} the {} was lost.", window[0], window[1])
}

/// Build a proverb end from a word.
fn proverb_end(word: &str) -> String {
    format!("And all for the want of a {word}.")
}
