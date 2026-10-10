pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.checked_ilog10().map_or(1, |d| d + 1);
    let target = num as u64;

    std::iter::successors(Some(num), |&n| (n >= 10).then_some(n / 10))
        .map(|n| ((n % 10) as u64).pow(digits))
        .try_fold(0u64, |sum, term| {
            let sum = sum + term;
            (sum <= target).then_some(sum)
        })
        == Some(target)
}
