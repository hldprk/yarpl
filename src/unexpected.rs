use crate::Expect;
use crate::Parser;
use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use std::any::Any;

/// An `Error` type returned from a failed parse.
#[derive(Debug, Clone)]
pub struct Unexpected {
	 
	parser: Parser,

}


impl From<Parser> for Unexpected {

	fn from(parser: Parser) -> Self {
		
		Self { parser }

	}

}

impl Display for Unexpected {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		let mut i = 0;
		
		let mut line_number = 0;

		let mut start_of_line = 0;

		for character in self.parser.input().chars() {

			if character == '\n' { 
				
				line_number += 1; 
			
				start_of_line = self.parser.index();

			}

			if i == self.parser.index() { break }

			else { i+= 1; }

		}

		let position_in_line = self.parser.index() - start_of_line;

		write!(f, "Expected '{}' at character {} on line {}.", self.parser.history().clone().pop().unwrap(), position_in_line, line_number)

	}

}

impl std::error::Error for Unexpected {

	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {

		None
	
	}	

}