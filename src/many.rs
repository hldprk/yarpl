use std::marker::PhantomData;
use std::ops::Range;
use std::fmt::Debug;

use crate::*;

/// Matches as many `T::Target` as possible. If one or more are parsed, returns them in a `Vec<T::Target>`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Many<T : Parse> { phantom_data: PhantomData<T> }

impl<T : Parse> Parse for Many<T> {

	type Target = Vec<T::Target>;

	fn parse_from(parser: &mut Parser) -> Result<Self::Target> {
	
		let mut results = Vec::default(); 

		loop {

			let ref mut cloned_parser = parser.clone();

			let result_maybe = T::parse_from(cloned_parser);

			if result_maybe.is_ok() {
				
				parser.clone_from(&cloned_parser);

				results.push(result_maybe.unwrap());
				
			} 
			
			else { break; }

		}
	
		if results.len() > 0 { Ok(results) } 
		
		else { Err(Error::new::<Self>(parser)) }

	}

}