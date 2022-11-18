#[must_use]
pub fn reply(message: &str) -> &str {
    let message = message.trim_end();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let shouting =
        !message.chars().any(char::is_lowercase) && message.chars().any(char::is_alphabetic);
    let questioning = message.ends_with('?');

    match (shouting, questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever.",
    }
}
