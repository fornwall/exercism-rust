#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (
        contains(first_list, second_list),
        contains(second_list, first_list),
    ) {
        (true, true) => Comparison::Equal,
        (false, true) => Comparison::Sublist,
        (true, false) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
