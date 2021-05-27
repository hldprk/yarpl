use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Range;

use crate::*;

/// A `Iterator<Item = char>` struct for parsing. 
#[derive(Clone, Debug)]
pub struct Parser {

	pub(crate)string: String,
	pub(crate)index: usize,

}

impl Iterator for Parser {

	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		
		let result = self.string.chars().nth(self.index);
		
		self.index += 1;

		result

	}

}


impl<T : Display> From<T> for Parser {

	fn from(other: T) -> Self {
		
		Parser {

			string: other.to_string(),
			index: 0

		}

	}

} 

impl Parser {

	/// If the type `T : Expect` can be parsed from the source string, returns `Ok(T)`.   
	pub fn expect<T: Expect>(&mut self) -> Result<T> {

		T::expect_from(self)
	
	}

	/// Expects a number of `T : Expect` within a given range, and returns `Ok(Vec<T>)` if successful.
	pub fn expect_many<T : Expect>(&mut self, range: Range<usize>) -> Result<Vec<T>>
	where Self : Sized + Debug {

		let mut i = 0;

		let mut results = Vec::default(); 

		loop {

			let result_maybe = T::expect_from(self);

			if result_maybe.is_ok() {
				
				results.push(result_maybe.unwrap());
				
				i += 1;

			} else {

				break;

			}

		}
	
		if range.contains(&i) {

			Ok(results)

		} else {

			Err(Unexpected::from(self.clone()))

		}

	}

	/// Expects `T : Expect`, and returns `Ok(Some(T))` if successful, `Ok(None)` otherwise.
	pub fn expect_maybe<T : Expect>(&mut self) -> Result<Option<T>> 
	where Self : Sized + Debug {
		
		let succeeds = T::expect_from(&mut self.clone()).is_ok();

		if succeeds {

			let expected = T::expect_from(self).unwrap();

			Ok(Some(expected))
			
		}
		
		else { Ok(None) }

	}

}

