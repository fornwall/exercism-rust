#[must_use]
pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut in_word = false;
    let mut last_was_lowercase = false;

    for c in phrase.chars() {
        let is_word_character = c.is_ascii_alphanumeric() || c == '\'';

        let push_this = if is_word_character {
            if in_word {
                last_was_lowercase && c.is_uppercase()
            } else {
                true
            }
        } else {
            false
        };

        if push_this {
            acronym.push(c.to_ascii_uppercase());
        }

        in_word = is_word_character;
        last_was_lowercase = c.is_lowercase();
    }

    acronym
}
