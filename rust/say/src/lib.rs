pub fn encode(n: u64) -> String {
    const BASE: u64 = 10;
    let places: Vec<u64> = get_places(n, BASE);
    


    "to finish".to_string()
}

fn get_places(n: u64, base: u64) -> Vec<u64> {
    let mut q: u64 = 10;
    let mut r: u64 = 0;
    let mut v: Vec<u64>  = Vec::new();

    while  q <= n {
        r = n % q - r;
        v.push(r);
        q = q*base;
    }
    v.push(n - v.iter().sum::<u64>());
    v.reverse();
    v
}

fn get_trailing_zeroes(n: u64) -> u64 {
   (n.to_string().len() as u64) - 1 
    
}

fn get_string_for_place(n: u64, base: u64) -> String {
    "".to_string()    
}
