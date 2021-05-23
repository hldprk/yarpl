use std::fmt::Display;
use std::fmt::Formatter;

use crate::Expect;

/// Parsed from one or more alphabetic characters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letters(pub(crate) String);

impl Expect for Letters {

	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized {
		
		let string: String = iterator.clone().take_while(|character| character.is_alphabetic()).collect();

		if string.is_empty() { Err(()) }

		else {

			iterator.advance_by(string.len());

			Ok(Letters(string))

		}
	
	}

}

impl Display for Letters {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}
