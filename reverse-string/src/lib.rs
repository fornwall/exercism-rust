use unicode_segmentation::UnicodeSegmentation;

#[must_use]
pub fn reverse(input: &str) -> String {
    input
        .graphemes(true)
        .rev()
        .fold(String::new(), |mut state, grapheme| {
            state.push_str(grapheme);
            state
        })
}
