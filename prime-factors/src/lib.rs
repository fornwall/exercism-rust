#[must_use]
pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut remainder = n;
    let mut p = 2;

    while remainder >= p {
        if remainder % p == 0 {
            primes.push(p);
            remainder /= p;
        } else {
            p += 1;
        }
    }

    primes
}
