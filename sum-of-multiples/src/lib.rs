pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .into_iter()
        .filter(|&i| {
            factors
                .iter()
                .filter(|&&factor| factor > 0)
                .any(|factor| i % factor == 0)
        })
        .sum()
}
