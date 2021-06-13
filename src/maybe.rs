use std::marker::PhantomData;
use std::fmt::Debug;

pub use crate::*;

/// Returns `Ok(Some(Self::Target))` if parsed successfully, `Ok(None)` otherwise. 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Maybe< T : Parse> { phantom_data: PhantomData<T> }

impl<T : Parse> Parse for Maybe<T> {

	type Target = Option<T::Target>;

	fn parse_from(parser: &mut Parser) -> Result<Self::Target> {
		
		let ref mut cloned_parser = parser.clone();

		let result = T::parse_from(cloned_parser);
		
		if result.is_ok() {

			Ok(Some(T::parse_from(parser).unwrap()))
			
		}
		
		else { Ok(None) }

	}

}