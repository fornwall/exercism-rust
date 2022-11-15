// https://github.com/rust-lang/rust-clippy/pull/9454
#![allow(clippy::use_self)]

use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    u32::from(color.int_value())
}

pub fn value_to_color_string(value: u32) -> String {
    let color = match ResistorColor::from_int(value.try_into().unwrap()) {
        Ok(value) => value,
        Err(_) => {
            return "value out of range".to_string();
        }
    };
    format!("{color:?}")
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
