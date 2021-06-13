use std::marker::PhantomData;
use std::fmt::Debug;

use crate::*;

/// Returns `Ok(())` if successful, `Err(Error)` otherwise. Doesn't consume input.
#[derive(Debug, Clone, Copy, Default)]
pub struct Must<T : Parse> {

	phantom_data: PhantomData<T>,

}

impl<T : Parse> Parse for Must<T> {

	type Target = ();

	fn parse_from(parser: &mut Parser) -> Result<Self::Target> {
		
		match parser.clone().parse::<T>() {

			Ok(_) => Ok(()),
			Err(_) => Err(Error::new::<Self>(parser))

		}

	}

}