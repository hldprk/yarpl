use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::*;

/// Parsed from a `Parser` pointing to one or more alphabetic characters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letters(pub(crate) String);

impl Expect for Letters {

	fn expect_from(parser: &mut Parser) -> Result<Self>
	where Self : Sized + Debug {
			
		let string: String = parser.clone().take_while(|character| character.is_alphabetic()).collect();

		if string.is_empty() { Err(Unexpected::from(parser.clone())) }

		else {

			parser.advance_by(string.len());

			Ok(Letters(string))

		}
	
	}

}

impl Display for Letters {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}
