const FACTOR_WORDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let sound = FACTOR_WORDS
        .iter()
        .filter_map(|(divisor, word)| (n % divisor == 0).then_some(*word))
        .collect::<String>();
    sound.is_empty().then(|| n.to_string()).unwrap_or(sound)
}
