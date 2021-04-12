use std::fmt::Debug;
use crate::feed::Feed;
use crate::feed::Result;


/// Holds tokens (`String`) that have been parsed, as well as remaining input.
#[derive(Clone, Debug, PartialEq)]
pub struct Consumer {

	taken: Vec<String>,
	remainder: String

}

impl From<&str> for Consumer {

	fn from(string: &str) -> Self {
		
		Consumer {
			
			taken: Vec::<String>::default(),
			remainder: String::from(string)
			
		}
		
	}
	
}

impl Consumer {

	/// Shifts tokens according to some `Feed` implementation and returns `Ok(())` if successful, `Err(())` otherwise.
	pub fn consume(&mut self, feed: &mut dyn Feed) -> Result {

		feed.feed(self)

	}	

	/// Returns the tokens parsed by `self`.
	pub fn taken(&self) -> Vec<String> {

		self.taken.clone()

	}
	
	/// Returns the remaining input held by `self` that has yet to be parsed.
	pub fn remainder(&self) -> String {
		
		self.remainder.clone()

	}
	
	/// Returns the last token collected by this `Consumer`.
	pub fn top(&self) -> Option<String> {

		match self.taken.last() {

			Some(string) => Some(string.clone()),
			None => None

		}
		
	}

	/// Matches a string to the beginning of the remaining input and pushes it to `taken`.
	pub(crate) fn consume_str(&mut self, string: &str) -> Result {
		
		if self.remainder.starts_with(string) {
			
			self.remainder = self.remainder.split_at(string.len()).1.to_string();
			
			self.taken.push(string.to_string());
			
			Ok(())
		}
		
		else { Err(()) }
		
	}

	
}
