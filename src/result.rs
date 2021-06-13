use std::fmt::Debug;
use crate::*;

/// The type returned from `Parser::parse`.
pub type Result<T> = std::result::Result<T, Error>;