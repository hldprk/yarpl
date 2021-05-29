use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::*;

/// When parsed, returns a `String` of one or more digits.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digits;

impl Expect for Digits {

	
	type Target = String;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target>
	where Self : Sized + Debug {

		let cloned_iterator = parser.clone();

		let string: String = cloned_iterator.take_while(|character| character.is_digit(10)).collect();

		if string.is_empty() { Err(Unexpected::from(parser.clone())) }

		else {

			parser.advance_by(string.len());

			Ok(string)

		}

	}

}
