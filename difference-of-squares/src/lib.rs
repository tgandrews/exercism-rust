pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).into_iter().fold(0, |acc, v| acc + v).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
        .into_iter()
        .map(|v| v.pow(2))
        .fold(0, |acc, v| acc + v)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
