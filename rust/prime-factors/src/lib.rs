pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut m: u64 = n;
    let mut p: u64 = 2;

    while m > 1 && p <= n {
        if m % p == 0 {
            m = m / p;
            factors.push(p);
        } else {
            p += 1;
        }
    }

    factors
}
