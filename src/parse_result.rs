
use crate::progress::Progress;
use crate::parse_error::ParseError;

/// Return type of `yarpl_fn` macro.
pub type ParseResult = Result<Progress, ParseError>;
