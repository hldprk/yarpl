use std::marker::PhantomData;

use crate::Parser;
use crate::Consume;
use crate::Result;


#[derive(Clone, Debug, Default, Copy)]
/// A type that, when parsed, ensures an object of type `T : Consume` *cannot be parsed* from some `Parser`.
pub struct Not<T : Consume>(PhantomData<T>);


impl<T : Consume + Default + Copy> Consume for Not<T> {

	type Target = ();

	fn consume(parser: &mut Parser) -> Result<Self::Target> {
		
		let ref mut parser_cloned = parser.clone();
		
		match parser_cloned.feed::<T>() {

			Ok(_) => Ok(()),
			Err(_) => Err(()),

		}

	}

}