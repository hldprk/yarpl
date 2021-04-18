
use crate::consume::Consume;
use crate::Parser;
use crate::consume::Result;
use std::marker::PhantomData;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;
#[derive(Clone, PartialEq, Default)]

/// Given a type `T : Consume` and a range `n .. m`, parses `T` between `n` and `m` times.
pub struct Many<T : Consume, const RANGE: Range<usize>>(PhantomData<T>);

impl <T : Consume, const RANGE: Range<usize>> Consume for Many<T, RANGE> {

	type Target = Vec<T::Target>;

	fn consume(parser: &mut Parser) -> Result<Self::Target> {
		
		let mut children = Vec::<T::Target>::default(); 

		let mut i = 0;

		loop {

			if i == RANGE.end { break; }

			let current_result = parser.feed::<T>();

			if current_result.is_ok() { 
				
				children.push(current_result.unwrap()); 
			
				i += 1;

			}

			else { break; }

		}

		if RANGE.contains(&i) {

			Ok(children)

		}
		
		else { Err(()) }

	}

}
