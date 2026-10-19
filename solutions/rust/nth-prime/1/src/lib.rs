/// Returns the 0-based nth prime number.
///
/// # Examples
///
/// ```
/// use nth_prime::nth;
///
/// assert_eq!(nth(0), 2);
/// assert_eq!(nth(1), 3);
/// assert_eq!(nth(5), 13);
/// ```
pub fn nth(n: u32) -> u32 {
    // Handle 2 separately so the rest of the search can skip all even numbers.
    if n == 0 {
        2
    } else {
        (3..)
            .step_by(2)
            .filter(|&x| is_odd_prime(x))
            // Already counted prime 2, so take the (n - 1)th odd prime.
            .nth((n - 1) as usize)
            .unwrap()
    }
}

/// Trial-divides an odd candidate by odd divisors up to √x.
///
/// Caller must pass an odd `x` greater than 2.
fn is_odd_prime(x: u32) -> bool {
    !(3..=((x as f64).sqrt() as u32))
        .step_by(2)
        .any(|i| x.is_multiple_of(i))
}
