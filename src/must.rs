use std::marker::PhantomData;
use std::fmt::Debug;

use crate::*;

/// Used to propogate the result of a parse without the `Parser` consuming input.
#[derive(Debug, Clone, Copy, Default)]
pub struct Must<T : Expect> {

	phantom_data: PhantomData<T>

}

impl<T : Expect> Expect for Must<T> {

	type Target = ();

	fn expect_from(parser: &mut Parser) -> Result<Self::Target>
	where Self : Sized + Debug {
		
		match parser.clone().expect::<T>() {

			Ok(_) => Ok(()),
			Err(_) => Err(Unexpected::from(parser.clone()))

		}

	}

}