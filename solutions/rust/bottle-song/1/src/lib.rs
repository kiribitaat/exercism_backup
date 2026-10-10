use core::fmt;

const NUMBERS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

struct Capitalize<'a>(&'a str);

impl<'a> fmt::Display for Capitalize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = self.0.chars();
        if let Some(first_letter) = chars.next() {
            write!(f, "{}", first_letter.to_ascii_uppercase())?;
            write!(f, "{}", chars.as_str())?;
        }
        Ok(())
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn verse(bottles: u32) -> String {
    let phrase = bottles_phrase(bottles);
    format!(
        "{} hanging on the wall,\n\
{} hanging on the wall,\n\
And if one green bottle should accidentally fall,\n\
There'll be {} hanging on the wall.",
        Capitalize(&phrase),
        Capitalize(&phrase),
        bottles_phrase(bottles - 1),
    )
}

fn plural_suffix(bottles: u32) -> &'static str {
    if bottles != 1 { "s" } else { "" }
}

fn bottles_phrase(bottles: u32) -> String {
    format!(
        "{} green bottle{}",
        NUMBERS[bottles as usize],
        plural_suffix(bottles)
    )
}
