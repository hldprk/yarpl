use std::any::type_name;
use std::borrow::BorrowMut;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::format;
use std::iter::FromIterator;
use std::ops::Range;
use std::str::Chars;

use crate::*;
  
/// Struct to facilitate parsing.
#[derive(Clone, Debug)]
pub struct Parser {

	input: String,
	history: Vec<String>,
	index: usize,
	pub should_skip_whitespace: bool

}

impl<T : Display> From<T> for Parser {

	fn from(other: T) -> Self {
		
		Parser {

			input: other.to_string(),
			history: Vec::default(),
			index: 0,
			should_skip_whitespace: false 

		}

	}

}

impl Iterator for Parser {

	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		
		let result = self.input.chars().nth(self.index);
		
		self.index += 1;

		result

	}

}



impl Parser {

	/// If the type `T : Expect` can be parsed from the source string, returns `Ok(T::Target)`.   
	pub fn expect<T: Expect>(&mut self) -> Result<T::Target> {

		self.history.push(type_name::<T>().to_string());
		
		if self.should_skip_whitespace { Maybe::<Spaces>::expect_from(self); }
		
		let result = T::expect_from(self);
		
		if self.should_skip_whitespace && result.is_ok() { Maybe::<Spaces>::expect_from(self); }

		result

	}
	
	pub fn input(&self) -> String {

		self.input.to_string()

	}

	pub fn index(&self) -> usize {

	
		self.index

	}

	pub fn history(&self) -> Vec<String> {

		self.history.clone()

	}

}

