use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
    let word = "ΑΒΓ";

    let inputs = ["ΑΒγ"];

    let outputs = vec![];

    let result = anagram::anagrams_for(word, &inputs);

    let expected: HashSet<&str> = HashSet::from_iter(outputs.iter().cloned());

    assert_eq!(result, expected);
}
