pub const fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub const fn sum_of_squares(n: u32) -> u32 {
    // https://www.cuemath.com/algebra/sum-of-squares/
    n * (n + 1) * (2 * n + 1) / 6
}

pub const fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
