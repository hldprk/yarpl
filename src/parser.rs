use std::any::type_name;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::format;
use std::ops::Range;

use crate::*;

/// A `Iterator<Item = char>` struct for parsing. 
#[derive(Clone, Debug)]
pub struct Parser {

	pub(crate) string: String,
	pub(crate) index: usize,
	pub(crate) history: Vec<String>


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
			index: 0,
			history: Vec::default()

		}

	}

} 

impl Parser {

	/// If the type `T : Expect` can be parsed from the source string, returns `Ok(T::Target)`.   
	pub fn expect<T: Expect>(&mut self) -> Result<T::Target> {

		self.history.push(type_name::<T>().to_string());
		
		T::expect_from(self)
	
	}

}

