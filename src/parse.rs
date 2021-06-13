
use std::any::Any;
use std::ops::Range;
use crate::*;
use std::fmt::Debug;

/// A `Parse` may use a `Parser` to yield an associated `Target` type.
pub trait Parse : Debug + Clone + Any {

	/// The type returned from parsing `Self`.
	type Target;

	/// What a parser will call to advance itself.
	fn parse_from(parser: &mut Parser) -> Result<Self::Target>;

}
