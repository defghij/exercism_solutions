pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut m = vec![0; 0];
    for x in factors.iter(){

        if 0 < *x && *x < limit {
            let mut n: u32 = 1;
            let mut y = n * x;

            while y < limit {

                if !m.contains(&y) { 
                    (m).push(y);
                }
                n += 1;
                y = n * x;
            }
        } else {
            (m).push(0);
        }
    }
    m.iter().sum::<u32>()
}
