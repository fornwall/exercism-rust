// &'static is a "lifetime specifier", something you'll learn more about later
#[must_use]
pub const fn hello() -> &'static str {
    "Hello, World!"
}
