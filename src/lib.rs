
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]
#![feature(const_generics)]
#![doc = include_str!("../README.md")]
#![feature(iter_advance_by)]
#![feature(in_band_lifetimes)]

mod expect;
mod letters;
mod digits;
mod spaces;
mod number;
mod just;
mod parser;
mod unexpected;
mod result;
mod must;
mod over;
mod under;
mod maybe;

pub use expect::Expect;
pub use letters::Letters;
pub use digits::Digits;
pub use spaces::Spaces;
pub use number::Number;
pub use just::Just;
pub use unexpected::Unexpected;
pub use parser::Parser;
pub use result::Result;
pub use over::Over;
pub use under::Under;
pub use maybe::Maybe;