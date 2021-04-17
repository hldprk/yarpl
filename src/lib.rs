
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]

#![doc = include_str!("../README.md")]


mod consumer;
mod feed;
mod many;
mod maybe;
mod not;
mod must;
mod peek;

#[macro_use]
mod only;

#[macro_use]
mod plan;


pub use consumer::Consumer;
pub use feed::Result;
pub use feed::Feed;
pub use many::Many;
pub use maybe::Maybe;
pub use not::Not;
pub use must::Must;
pub use peek::Peek;
