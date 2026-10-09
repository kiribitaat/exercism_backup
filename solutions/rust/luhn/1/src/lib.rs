/// Check a Luhn checksum.
///
/// Invalid inputs (anything non-digit, or fewer than 2 digits after stripping whitespace) should return false.
///
/// # Arguments
///
/// * `code` - The code to check.
///
/// # Examples
///
/// ```
/// use luhn::is_valid;
///
/// assert!(is_valid("055 444 285"));
/// assert!(!is_valid("055a 444 285"));
/// assert!(!is_valid(""));
/// assert!(!is_valid("1"));
/// ```
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .try_fold((0u32, 0usize), |(sum, _), (index, ch)| {
            let digit = ch.to_digit(10)?;

            let value = if index % 2 == 1 {
                double_digit(digit)
            } else {
                digit
            };

            Some((sum + value, index + 1))
        })
        .is_some_and(|(sum, length)| length > 1 && sum % 10 == 0)
}

fn double_digit(digit: u32) -> u32 {
    let doubled = digit * 2;
    if doubled > 9 { doubled - 9 } else { doubled }
}
