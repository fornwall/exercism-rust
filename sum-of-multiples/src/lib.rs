#[must_use]
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|n| factors.iter().any(|f| *f != 0 && n % f == 0))
        .sum()
}
