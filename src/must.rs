use std::marker::PhantomData;

use crate::Parser;
use crate::Consume;
use crate::Result;

#[derive(Clone, Debug, Default, Copy)]
/// When parsed, ensures an object of type `T : Consume` *is able* to be parsed from a parser, but will consume no input.
pub struct Must<T : Consume>(PhantomData<T>);


impl<T : Consume> Consume for Must<T> {

	type Target = ();

	fn consume(parser: &mut Parser) -> Result<Self::Target> {
		
		let ref mut parser_cloned = parser.clone();
		
		match parser_cloned.feed::<T>() {

			Ok(_) => Ok(()),
			Err(_) => Err(()),

		}
	}

}