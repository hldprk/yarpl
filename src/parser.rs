use crate::parse_result::ParseResult;

/// An alias for the function signature of `yarpl` macros.
pub type Parser = fn(String, usize) -> ParseResult;
