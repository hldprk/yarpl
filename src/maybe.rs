use std::marker::PhantomData;

pub use crate::*;

/// Returns `Ok(Some(Self::Target))` if parsed successfully, `Ok(None)` otherwise. 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Maybe<T : Expect> { phantom_data: PhantomData<T> }

impl<T : Expect> Expect for Maybe<T> {

	type Target = Option<T::Target>;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target> {
		
		let succeeds = T::expect_from(&mut parser.clone()).is_ok();

		if succeeds {

			let expected = T::expect_from(parser).unwrap();

			Ok(Some(expected))
			
		}
		
		else { Ok(None) }

	}

}