use std::cmp::Ordering;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn find<T: PartialOrd + Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();

    let mut low = 0;
    let mut high = array.len().checked_sub(1)?;

    while low <= high {
        let middle = low + (high - low) / 2;

        match array[middle].cmp(&key) {
            Ordering::Less => {
                low = middle.checked_add(1)?;
            }
            Ordering::Equal => {
                return Some(middle);
            }
            Ordering::Greater => {
                high = middle.checked_sub(1)?;
            }
        }
    }

    None
}
