use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let anagrams = possible_anagrams
        .iter()
        .filter(|x| is_nontrivial_anagram(word, x))
        .copied()
        .collect();
    anagrams
}

fn is_nontrivial_anagram(word_one: &str, word_two: &str) -> bool {
    to_sorted_vec(&word_one.to_lowercase()) == to_sorted_vec(&word_two.to_lowercase())
        && word_one.to_lowercase() != word_two.to_lowercase()
}

fn to_sorted_vec<'a>(word: &str) -> Vec<char> {
    let mut s_word: Vec<char> = word.chars().collect();
    s_word.sort();
    s_word
}
