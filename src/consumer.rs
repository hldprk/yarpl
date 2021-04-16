use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use crate::feed;
use crate::feed::Feed;
use crate::feed::Result;


/// Holds tokens (`String`) that have been parsed, as well as remaining input.
#[derive(Clone, Debug, PartialEq)]
pub struct Consumer {

	pub(crate) tokens: Vec<String>,
	pub(crate) remainder: String,

}

impl From<&str> for Consumer {

	fn from(string: &str) -> Self {
		
		Consumer {
			
			tokens: Vec::<String>::default(),
			remainder: String::from(string),

		}
		
	}
	
}

impl Consumer {

	/// Shifts tokens according to some `Feed` implementation and returns `Ok(())` if successful, `Err(())` otherwise.
	pub fn consume(&mut self, feed: &mut dyn Feed) -> Result {

		feed.feed(self)

	}	

	/// Returns the tokens parsed by `self`.
	pub fn tokens(&self) -> Vec<String> {

		self.tokens.clone()

	}
	
	/// Returns the remaining input held by `self` that has yet to be parsed.
	pub fn remainder(&self) -> String {
		
		self.remainder.clone()

	}
	
	/// Returns the last token collected by this `Consumer`.
	pub fn top(&self) -> Option<String> {

		match self.tokens.last() {

			Some(string) => Some(string.clone()),
			None => None

		}
		
	}

}

