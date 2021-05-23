use std::fmt::Display;
use std::fmt::Formatter;

use crate::Expect;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digits(pub(crate) String);


/// Parsed from one or more numeric characters.
impl Expect for Digits {

	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized {
		
		let cloned_iterator = iterator.clone();

		let string: String = cloned_iterator.take_while(|character| character.is_digit(10)).collect();

		if string.is_empty() { Err(()) }

		else {

			iterator.advance_by(string.len());

			Ok(Digits(string))

		}

	}

}

impl Display for Digits {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}