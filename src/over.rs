use std::marker::PhantomData;
use std::ops::Range;
use std::fmt::Debug;

use crate::*;

/// Returns `Vec<T::Target>` if it can be parsed more than the amount specified, `Err` otherwise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Over<T : Expect, const N : usize> { phantom_data: PhantomData<T> }

impl<T : Expect, const N : usize> Expect for Over<T, N> {

	type Target = Vec<T::Target>;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target> {
		
		let ref mut parser_clone = parser.clone();

		let range = N..;
		
		let mut i = 0;

		let mut results = Vec::default(); 

		loop {

			let result_maybe = T::expect_from(parser_clone);

			if result_maybe.is_ok() {
				
				results.push(result_maybe.unwrap());
				
				i += 1;

			} else {

				break;

			}

		}
	
		if range.contains(&i) {

			parser.clone_from(&parser_clone);

			Ok(results)

		} else {

			Err(Unexpected::from(parser_clone.clone()))

		}

	}

}