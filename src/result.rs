use std::fmt::Debug;
use crate::Expect;
use crate::Parser;
use crate::Unexpected;

/// An alias for `Result` which is returned from methods of `Parser`.
pub type Result<T> = std::result::Result<T, Unexpected<T>>;