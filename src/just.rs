use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;

use crate::*;

/// A generic way to parse a specific string.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Just<const STRING: &'static str>;

impl<const STRING: &'static str> Expect for Just<STRING> {

	type Target = &'static str;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target> {
			
		let cloned_iterator = parser.clone();

		let remainder: String = cloned_iterator.collect();

		if remainder.starts_with(STRING) {

			parser.advance_by(STRING.len());

			Ok(STRING)

		} else {

			Err(Unexpected::from(parser.clone()))

		}
		
	}

}
