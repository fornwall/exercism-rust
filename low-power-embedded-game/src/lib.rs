pub const fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter_map(|(index, element)| (index % 2 == 0).then_some(element))
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub const fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
