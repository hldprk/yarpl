use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;

use crate::*;

/// Used by a `Parser` to match a specific, provided string.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Just<const STRING: &'static str>;

impl<const STRING: &'static str> Expect for Just<STRING> {

	fn expect_from(parser: &mut Parser) -> Result<Self>
	where Self : Sized + Debug {
			
		let cloned_iterator = parser.clone();

		let remainder: String = cloned_iterator.collect();

		if remainder.starts_with(STRING) {

			parser.advance_by(STRING.len());

			Ok(Just::<STRING>)

		} else {

			Err(Unexpected::from(parser.clone()))

		}
		
	}

}

impl<const STRING: &'static str> Display for Just<STRING> {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", STRING)

	}

}