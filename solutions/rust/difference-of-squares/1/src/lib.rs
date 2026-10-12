pub fn square_of_sum(n: u32) -> u32 {
    let n = u64::from(n);
    let sum = n * (n + 1) / 2;
    sum.pow(2) as u32
}

pub fn sum_of_squares(n: u32) -> u32 {
    let n = u64::from(n);
    let sum_sq = n * (n + 1) * (2 * n + 1) / 6;
    sum_sq as u32
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
