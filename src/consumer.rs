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
	///
	/// # Example
	/// 
	/// ```
	///	# use yarpl::Consumer;
	///
	/// let input = "aaabbbccc";
	///
	/// let ref mut consumer = Consumer::from(input);
	/// 
	/// consumer.shift("aaa");
	/// consumer.shift("bbb");
	/// consumer.shift("ccc");
	/// 
	/// let expected = vec![String::from("aaa"), String::from("bbb"), String::from("ccc")];
	///
	///	assert_eq!(consumer.taken() , expected);
	/// ``` 

	pub fn shift(&mut self, string: &str) -> Result {
		
		if self.remainder.starts_with(string) {
			
			self.remainder = self.remainder.split_at(string.len()).1.to_string();

			self.taken.push(string.to_string());

			Ok(())
		}
		
		else { Err(()) }

	}

	/// Matches a token built according to the provided `char` predicate.
	///
	/// # Example
	/// 
	/// ```
	///	# use yarpl::Consumer;
	///
	/// let input = "1234";
	///
	/// let ref mut consumer = Consumer::from(input);
	/// 
	/// /// `consumer` will build a token from as many characters that are digits.
	/// consumer.shift_characters(&|c| "1234567890".contains(c));
	/// 
	/// /// The entire input was parsed because all characters were digits.
	///	assert_eq!(consumer.top().unwrap() , input.to_string());
	/// ``` 
	pub fn shift_characters(&mut self, function: &dyn Fn(char) -> bool) -> Result {
		
		let string : String = self.remainder.chars().take_while(
			|character |
				function(*character)).collect();
		
		self.shift(&string)

	}
	
	
}
