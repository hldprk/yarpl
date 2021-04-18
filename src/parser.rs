use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use crate::consume;
use crate::consume::Consume;
use crate::consume::Result;


/// Holds tokens (`String`) that have been parsed, as well as remaining input.
#[derive(Clone, Debug, PartialEq)]
pub struct Parser {

	pub(crate) tokens: Vec<String>,
	pub(crate) remainder: String,

}

impl From<&str> for Parser {

	fn from(string: &str) -> Self {
		
		Parser {
			
			tokens: Vec::<String>::default(),
			remainder: String::from(string),

		}
		
	}
	
}

impl Parser {

	/// Shifts tokens according to some `Consume` implementation and returns `Ok(())` if successful, `Err(())` otherwise.
	pub fn feed<T : Consume>(&mut self) -> Result<T::Target> {

		T::consume(self)

	}	

	/// Returns the tokens parsed by `self`.
	pub fn tokens(&self) -> Vec<String> {

		self.tokens.clone()

	}
	
	/// Returns the remaining input held by `self` that has yet to be parsed.
	pub fn remainder(&self) -> String {
		
		self.remainder.clone()

	}
	
	/// Returns the last token collected by this `Parser`.
	pub fn top(&self) -> Option<String> {

		match self.tokens.last() {

			Some(string) => Some(string.clone()),
			None => None

		}
		
	}

}

