use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_string(&lowercase_word);
    possible_anagrams
        .iter()
        .filter(|&anagram| {
            let lowercase_anagram = anagram.to_lowercase();
            sort_string(&lowercase_anagram) == sorted_word && lowercase_anagram != lowercase_word
        })
        .cloned()
        .collect()
}

fn sort_string(s: &str) -> Vec<char> {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars
}
