pub fn square_of_sum(n: u32) -> u32 {
    let v1: Vec<u32> = (1.. n+1).collect();
    let v1_sum:u32 =  v1.iter().sum();
    v1_sum * v1_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let v1: Vec<u32> = (1.. n+1).collect();
    let v2_squared: Vec<u32> = v1.iter().map(|x| x*x).collect();
    v2_squared.iter().sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n)-sum_of_squares(n)
}
