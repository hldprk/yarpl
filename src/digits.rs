use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::*;

/// Parsed from an `Iterator` pointing to one or more digits.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digits(pub(crate) String);

impl Expect for Digits {

	fn expect_from(parser: &mut Parser) -> Result<Self>
	where Self : Sized + Debug {

		let cloned_iterator = parser.clone();

		let string: String = cloned_iterator.take_while(|character| character.is_digit(10)).collect();

		if string.is_empty() { Err(Unexpected::from(parser.clone())) }

		else {

			parser.advance_by(string.len());

			Ok(Digits(string))

		}

	}

}

impl Display for Digits {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}
