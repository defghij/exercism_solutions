use high_scores::*;

pub fn main() {
    let scores_struct: HighScores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);

    for s in scores_struct.scores().iter() {
        println!("{:}", s);
    }

    println!("max: {}", scores_struct.personal_best().unwrap_or(0));

    println!("top three: ");
    for e in scores_struct.personal_top_three().iter() {
        println!("{}", e);
    }
}
