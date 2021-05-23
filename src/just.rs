use std::fmt::Display;
use std::fmt::Formatter;

use crate::Expect;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Just<const STRING: &'static str>;

impl<const STRING: &'static str> Expect for Just<STRING> {

	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized {
		
		let cloned_iterator = iterator.clone();

		let remainder: String = cloned_iterator.collect();

		if remainder.starts_with(STRING) {

			iterator.advance_by(STRING.len());

			Ok(Just::<STRING>)

		} else {

			Err(())

		}
		
	}

}

impl<const STRING: &'static str> Display for Just<STRING> {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", STRING)

	}

}