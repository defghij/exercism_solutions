pub fn check(candidate: &str) -> bool {
    let mut d: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    // Instead of doing this
    // d.iter().collect::HashSet<&char>();
    // since HashSets dont include duplicates
    d.sort_by(|a, b| b.cmp(a));
    d.dedup();


    d.len()
        == candidate
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .len()
}
