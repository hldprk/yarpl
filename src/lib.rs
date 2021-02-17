
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]


// The linter doesn't like this line, but it compiles fine.
#![doc = include_str!("../README.md")]


pub mod core;
pub mod parsers;
