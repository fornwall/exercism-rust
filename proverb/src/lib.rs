#[must_use]
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    let wants = list
        .windows(2)
        .map(|words| format!("For want of a {} the {} was lost.\n", words[0], words[1]))
        .collect::<String>();

    format!("{wants}And all for the want of a {}.", list[0])
}
