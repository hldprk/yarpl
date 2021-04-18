
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]
#![feature(const_generics)]
#![feature(const_fn)]
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]
#![doc = include_str!("../README.md")]


mod parser;
mod consume;
mod many;
mod maybe;
mod not;
mod must;
mod only;

pub use parser::Parser;
pub use consume::Result;
pub use consume::Consume;
pub use many::Many;
pub use maybe::Maybe;
pub use not::Not;
pub use must::Must;
pub use only::Only;