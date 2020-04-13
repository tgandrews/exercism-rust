pub fn factors(n: u64) -> Vec<u64> {
    let mut i = 2u64;
    let mut num_to_factor = n;
    let mut factors = vec![];
    loop {
        if num_to_factor == 1 {
            return factors;
        }
        if num_to_factor % i == 0 {
            factors.push(i);
            num_to_factor = num_to_factor / i;
        } else {
            i += 1;
        }
    }
}
