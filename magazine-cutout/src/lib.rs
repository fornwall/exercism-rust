use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_count = HashMap::<&str, u32>::new();
    for word in magazine {
        *magazine_word_count.entry(word).or_default() += 1;
    }
    for word in note {
        let count = magazine_word_count.entry(word).or_default();
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}
