#[must_use]
pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s), "Square must be between 1 and 64");
    2_u64.pow(s - 1)
}

#[must_use]
pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
