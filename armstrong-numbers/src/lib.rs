#[must_use]
pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    num_str
        .bytes()
        .map(|digit| (u32::from(digit - b'0')).pow(num_digits))
        .sum::<u32>()
        == num
}
