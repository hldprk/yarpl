use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::*;

/// Parses a `String` of one or more alphabetic characters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letters;

impl Expect for Letters {

	type Target = String;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target> {
			
		let string: String = parser.clone().take_while(|character| character.is_alphabetic()).collect();

		if string.is_empty() { Err(Unexpected::from(parser.clone())) }

		else {

			parser.advance_by(string.len());

			Ok(string)

		}
	
	}

}

