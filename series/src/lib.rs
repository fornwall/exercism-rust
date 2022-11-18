#[must_use]
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|window| window.iter().map(|&byte| byte as char).collect::<String>())
        .collect()
}
