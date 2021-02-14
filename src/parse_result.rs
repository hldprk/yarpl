
use crate::progress::Progress;
use crate::parse_error::ParseError;

/// Return type of `yarpl` macros.
pub type ParseResult = Result<Progress, ParseError>;
