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

	pub input: String,
	pub history: Vec<String>,
	pub index: usize

}

impl<T : Display> From<T> for Parser {

	fn from(other: T) -> Self {
		
		Parser {

			input: other.to_string(),
			history: Vec::default(),
			index: 0

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
		
		T::expect_from(self)
	
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

