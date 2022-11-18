use std::collections::BTreeMap;

#[must_use]
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&score, chars)| chars.iter().map(move |&c| (c.to_ascii_lowercase(), score)))
        .collect()
}
