const fn singular_or_plural<'a>(cardinality: u32, singular: &'a str, plural: &'a str) -> &'a str {
    if cardinality == 1 {
        singular
    } else {
        plural
    }
}

#[must_use]
pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string()
    } else {
        let n_minus_1 = if n == 1 {
            "no more".to_string()
        } else {
            (n - 1).to_string()
        };

        let n_bottle = singular_or_plural(n, "bottle", "bottles");
        let it_or_one = singular_or_plural(n, "it", "one");
        let n_minus_1_bottle = singular_or_plural(n - 1, "bottle", "bottles");

        format!("{n} {n_bottle} of beer on the wall, {n} {n_bottle} of beer.\n\
        Take {it_or_one} down and pass it around, {n_minus_1} {n_minus_1_bottle} of beer on the wall.\n")
    }
}

#[must_use]
pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .fold(String::new(), |state, str| {
            format!("{state}{}{str}", if state.is_empty() { "" } else { "\n" })
        })
}
