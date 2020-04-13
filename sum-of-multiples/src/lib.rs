pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .into_iter()
        .filter(|i| {
            for factor in factors {
                if factor == &0 {
                    continue;
                }
                if i % factor == 0 {
                    return true;
                }
            }
            return false;
        })
        .sum()
}
