#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    while haystack.len() >= needle.len() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    false
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
