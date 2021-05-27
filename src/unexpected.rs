use crate::Expect;
use crate::Parser;
use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use std::any::Any;

/// An `Error` type returned from a failed parse.
#[derive(Debug, Clone)]
pub struct Unexpected<T> {
	 
	parser: Parser,
	phantom_data: PhantomData<T>
	
}


impl<T> From<Parser> for Unexpected<T> {

	fn from(parser: Parser) -> Self {
		
		Self {

			parser,
			phantom_data: PhantomData::<T>

		}

	}

}

impl<T : Any + 'static> Display for Unexpected<T> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		



		write!(f, "Expected {} at {}.", type_name::<T>(), self.parser.index)

	}

}

impl<T : Any + Debug + 'static> std::error::Error for Unexpected<T> {

	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {

		None
	
	}	

}