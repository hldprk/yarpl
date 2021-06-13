
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]

#![feature(extended_key_value_attributes)]
#![feature(const_generics)]
#![feature(iter_advance_by)]

#![doc = include_str!("../README.md")]

mod parse;
mod error;
mod parser;
mod result;
mod either;
mod many;
mod maybe;
mod must;
mod literal;
mod pretty_type_name;

use pretty_type_name::pretty_type_name;

pub use parser::Parser;
pub use parse::Parse;
pub use error::Error;
pub use result::Result;
pub use either::Either;
pub use many::Many;
pub use maybe::Maybe;
pub use must::Must;
pub use literal::Literal;
