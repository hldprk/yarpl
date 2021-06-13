use std::any::Any;
use std::any::type_name;
use std::clone;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::format;
use std::iter::FromIterator;
use std::ops::Range;
use std::str::Chars;
use std::usize;

use crate::*;

/// Is used to parse things.
#[derive(Clone, Debug)]
pub struct Parser<'a> {

	input: &'a str,
	pub(crate)iterator: Chars<'a>,
	strict_mode: bool

}

impl<'a> From<&'a str> for Parser<'a> {

	fn from(input: &'a str) -> Self {
		
		Parser {

			input,
			iterator: input.chars(),
			strict_mode: true

		}

	}

}


impl<'a> Parser<'a> {

	/// Parses `T : FromStr` from the given input.
	pub fn parse<T: Parse>(&mut self) -> Result<T::Target> {

		type Spaces = Many<Either<"\t\n\r ">>;

		let ref mut cloned_parser = self.clone();

		if !(self.strict_mode) { let _ = Maybe::<Spaces>::parse_from(cloned_parser); }
		
		let result = T::parse_from(cloned_parser);
		
		if !(self.strict_mode) { let _ = Maybe::<Spaces>::parse_from(cloned_parser); }
		
		if result.is_ok() { self.clone_from(cloned_parser); } 
		
		result

	}

	/// Returns the entire input string.
	pub fn input(&self) -> &'a str {

		self.input

	}

	/// Returns the current index of this `Parser`.
	pub fn index(&self) -> usize {

 		self.iterator.clone().rev().count()

	}

	/// Returns the current line number.
	pub fn line(&self) -> usize {
		
		let enumerated = self.input.char_indices();

		let mut result = 0;

		for (i, character) in enumerated {

			if character == '\n' { result += 1; }
			
			if i > self.index() { break; }

		}

		if result == 0 { result } else { result - 1 }

	}

	/// The current position within the current line.
	pub fn column(&self) -> usize {

		let beginning_reversed = self.input.clone().split_at(self.index()).1.chars().rev().collect::<String>();

		beginning_reversed.chars().position(|c| c == '\n').unwrap_or(0)
		
	}

	pub fn enable_strict_mode(&mut self) {
		
		self.strict_mode = true;
		
	} 

	pub fn disable_strict_mode(&mut self) {

		self.strict_mode = false;

	}


}
