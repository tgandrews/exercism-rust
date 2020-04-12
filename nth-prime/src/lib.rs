fn is_prime(primes: &Vec<u32>, number: u32) -> bool {
    for prime in primes {
        if number % prime == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    for i in 2..u32::max_value() {
        if is_prime(&primes, i) {
            if primes.len() == n as usize {
                return i;
            }
            primes.push(i);
        }
    }

    return 0;
}
