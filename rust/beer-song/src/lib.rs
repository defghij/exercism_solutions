pub fn verse(n: u32) -> String {
    match n {
        0 => {
            return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        }
        1 => {
            return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
        }
        _ => {
            return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n-1,(if n == 2 {""}else{"s"}));
        }
    }
}

pub fn sing(last_verse: u32, first_verse: u32) -> String {
    let verses: Vec<u32> = (std::ops::Range{start: first_verse, end: last_verse + 1}).rev().collect();
    verses.iter().map(|n| verse(*n)).collect::<Vec<String>>().join("\n")
}
