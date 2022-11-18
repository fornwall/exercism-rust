use std::collections::HashSet;

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let mut c1 = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut c2 = possible_anagram.to_lowercase().chars().collect::<Vec<_>>();

    if c1 == c2 {
        // "The solution cannot contain the input word. A word is always an
        // anagram of itself, which means it is not an interesting result."
        return false;
    }

    c1.sort_unstable();
    c2.sort_unstable();

    c1 == c2
}

#[must_use]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|p| is_anagram(word, p))
        .copied()
        .collect()
}
