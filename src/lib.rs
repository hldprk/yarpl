
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]
#![feature(const_generics)]
#![doc = include_str!("../README.md")]
#![feature(iter_advance_by)]

mod expect;
mod letters;
mod digits;
mod spaces;
mod number;
mod just;

pub use expect::Expect;
pub use letters::Letters;
pub use digits::Digits;
pub use spaces::Spaces;
pub use number::Number;
pub use just::Just;
