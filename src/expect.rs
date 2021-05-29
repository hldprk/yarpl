
use std::any::Any;
use std::ops::Range;
use crate::*;
use std::fmt::Debug;

/// Implements how a `Parser` should consume its input to yield `Self::Target`.
pub trait Expect : Debug + Clone + Any + 'static {

	/// The type expected from parsing `Self`.
	type Target;

	/// What a parser will call to advance itself. 
	fn expect_from(parser: &mut Parser) -> Result<Self::Target>;	

}
