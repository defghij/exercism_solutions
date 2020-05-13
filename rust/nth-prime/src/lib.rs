pub fn nth(n: u32) -> u32 {
    let mut primes_found: u32 = 0;
    let mut current_number: u32 = 2;

    while primes_found != n {
        current_number += 1;
        if sieve_test(current_number){
            primes_found += 1;
        }
    }
    return current_number;
}

pub fn sieve_test(n: u32) -> bool {
    for i in 2..n{
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
