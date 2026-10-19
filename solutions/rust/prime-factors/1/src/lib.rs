pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Peel off the only even prime first so the main loop can step by 2.
    while n.is_multiple_of(2) {
        factors.push(2);
        n /= 2;
    }

    // Odd candidates only. `d <= n / d` is the √n cutoff without overflowing `d * d`.
    let mut divisor = 3;
    while n > 1 && divisor <= n / divisor {
        while n.is_multiple_of(divisor) {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 2;
    }

    // Whatever remains (if anything) is a prime larger than √original-remaining.
    if n > 1 {
        factors.push(n);
    }

    factors
}
