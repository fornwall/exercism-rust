/// Check a Luhn checksum.
#[must_use]
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.replace(' ', "");

    if stripped_code.len() <= 1 || !stripped_code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    stripped_code
        .bytes()
        .rev()
        .enumerate()
        .map(|(idx, digit)| {
            let multiplier = 1 + u8::from(idx % 2 == 1);
            let digit_value = digit - b'0';
            let multiplied_value = multiplier * digit_value;
            let truncated_value = multiplied_value - 9 * u8::from(multiplied_value > 9);
            u32::from(truncated_value)
        })
        .sum::<u32>()
        % 10
        == 0
}
