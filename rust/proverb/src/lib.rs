pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: String = String::new();

    if list.len() == 0 {return proverb}
    for idx in 0..list.len()-1{
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", list[idx], list[idx+1]));
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
