use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use crate::Unexpected;
use crate::Expect;
use crate::Parser;
use crate::Result;

/// Parsed from a `Parser` pointing to one or more whitespace characters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spaces(pub(crate) String);

impl Expect for Spaces {

	fn expect_from(parser: &mut Parser) -> Result<Self>
	where Self : Sized + Debug {

		let cloned_iterator = parser.clone();

		let string: String = cloned_iterator.take_while(|character| character.is_whitespace()).collect();

		if string.is_empty() { Err(Unexpected::from(parser.clone())) }

		else {

			parser.advance_by(string.len());

			Ok(Spaces(string))

		}
	
	}

}

impl Display for Spaces {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}
