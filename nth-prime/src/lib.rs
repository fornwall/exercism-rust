#[must_use]
pub fn nth(n: u32) -> u32 {
    // https://stemhash.com/how-to-find-the-nth-prime-number/

    let nth_prime = n + 1;

    let mut biggest_candidate = std::cmp::max(3, 2 * nth_prime);
    let mut candidates = (2..biggest_candidate).collect::<Vec<_>>();
    let mut primes = vec![];

    loop {
        let sieve = candidates[0];

        primes.push(sieve);

        if primes.len() == nth_prime as usize {
            return sieve;
        }

        candidates.retain(|num| num % sieve != 0);

        if candidates.is_empty() {
            let new_biggest_candidate = biggest_candidate + nth_prime;
            candidates = (biggest_candidate..new_biggest_candidate)
                .filter(|num| primes.iter().all(|prime| num % prime != 0))
                .collect();
            biggest_candidate = new_biggest_candidate;
        }
    }
}
