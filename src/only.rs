use std::fmt::Formatter;
use std::ops::Deref;
use std::fmt::Display;

use crate::Consume;
use crate::Parser;
use crate::Result;


#[derive(Clone, Copy, Default)]
/// A `Consume` type from string literal.
pub struct Only<const STRING: &'static str>;
 

impl<const STRING: &'static str> Display for Only<STRING> {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", STRING)

	}

}

impl<const STRING: &'static str> Consume for Only<STRING> {

	type Target = &'static str;

	fn consume(parser: &mut Parser) -> Result<Self::Target> {
		
		let string = STRING.to_string();

		if parser.remainder.starts_with(&string) {
			
			parser.remainder = parser.remainder.split_at(string.len()).1.to_string();
			
			parser.tokens.push(string.to_string());
			
			Ok(STRING)
		}
		
		else { Err(()) }

	}

}