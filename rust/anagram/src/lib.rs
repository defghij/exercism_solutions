use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<& str> = HashSet::new();

    for poss_anagram in possible_anagrams.iter() {
        if is_nontrivial_anagram(word, poss_anagram) {
            anagrams.insert(poss_anagram);
        }
    }

    anagrams
}

fn is_nontrivial_anagram(word_one: &str, word_two: &str) -> bool {
    to_sorted_vec(&word_one.to_lowercase()) == to_sorted_vec(&word_two.to_lowercase())
    &&
    word_one.to_lowercase() != word_two.to_lowercase()
}

fn to_sorted_vec<'a>(word: &str) -> Vec<char> {
    let mut s_word: Vec<char> = word.chars().collect();
    s_word.sort();
    s_word
}
