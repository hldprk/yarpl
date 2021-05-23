use std::fmt::Display;
use std::fmt::Formatter;

use crate::Expect;
/// Parsed from an `Iterator` starting with one or more whitespace characters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spaces(pub(crate) String);

impl Expect for Spaces {

	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized {
		
		let cloned_iterator = iterator.clone();

		let string: String = cloned_iterator.take_while(|character| character.is_whitespace()).collect();

		if string.is_empty() { Err(()) }

		else {

			iterator.advance_by(string.len());

			Ok(Spaces(string))

		}
	
	}

}

impl Display for Spaces {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}
