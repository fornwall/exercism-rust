#[must_use]
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut step = 0;
    let mut number = n;

    while number != 1 {
        step += 1;
        number = match number % 2 {
            0 => number / 2,
            _ => number.checked_mul(3)?.checked_add(1)?,
        };
    }

    Some(step)
}
