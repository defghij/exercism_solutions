use saddle_points::*;

pub fn main() {
    
    let input = vec![vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];

    let sp: Vec<(usize, usize)>  = find_saddle_points(&input);

    println!("saddle points");
    sp.iter().for_each(|s| print!("{:?}", s));
    println!();


}
