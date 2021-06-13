
use crate::*;
use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::write;
use std::marker::PhantomData;
use std::any::Any;
use colored::*;

/// `Parser` returns this when unsuccessful. 
#[derive(Debug, Clone)]
pub struct Error {

	type_name: String,
	index: usize,
	line: usize,
	column: usize,
	offending_line: String

}

impl Error {

	pub fn new<T : Parse + Debug + Clone>(parser: &mut Parser) -> Error {

		Error {
			
			type_name: pretty_type_name::<T>(),
			index: parser.index(),
			line: parser.line(),
			column: parser.column(),
			offending_line: parser.input().lines().nth(parser.line()).unwrap().to_string()
		
		}

	}

}

impl Display for Error {

	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

		let padding = (0..self.column).map(|_| ' ').collect::<String>().red();
		
		writeln!(
			f,
			"\n{} Expected {} on line {}, column {}.",
			"Error!".bright_red(),
			self.type_name.bold(),
			self.line,
			self.column
		);

		writeln!(f, "{}", self.offending_line.dimmed());
		writeln!(f, "{}{}", padding, "^".bright_red())

	}

}

impl std::error::Error for Error {

	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {

		None
		
	}	

}