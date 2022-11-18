#[must_use]
pub const fn private_key(p: u64) -> u64 {
    (1 + p) / 2
}

fn multiply_modulo(a: u64, b: u64, modulus: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(modulus)) as u64
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    // https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply_modulo(result, base, modulus);
        }
        exp >>= 1;
        base = multiply_modulo(base, base, modulus);
    }
    result
}

#[must_use]
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

#[must_use]
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
