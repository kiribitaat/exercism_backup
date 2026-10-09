/// Simply reverse the string using the `chars` method
#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/// Advanced reverse the string using the `graphemes` method
#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    input.graphemes(true).rev().collect()
}
