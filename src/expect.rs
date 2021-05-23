use std::ops::RangeBounds;


/// Types that are `Expect` can be constructed by consuming strings with `expect_from`.
pub trait Expect {

	/// Consumes characters from a `yarpl::Iterator` to yield `Ok(Self)` if successful.
	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized;
	
	/// Expects a number of `Self` from a given range and returns `Result<Vec<Self>, ()>`.
	fn expect_many_from<I : Iterator<Item = char> + Clone, R : RangeBounds<usize>>(iterator: &mut I, range: R) -> Result<Vec<Self>, ()> where Self : Sized {

		let mut targets = Vec::<Self>::default();

		loop {

			let result = Self::expect_from(iterator);

			if result.is_ok() { targets.push(result.unwrap()) } 

			else { break; }

		}

		if range.contains(&targets.len()) { Ok(targets) }

		else { Err(()) }

	}

	/// Will return `Ok(Some(Self))` if successful, `Ok(None)` otherwise.
	fn expect_from_maybe<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Option<Self>, ()> where Self : Sized {
		
		let succeeds = Self::expect_from(&mut iterator.clone()).is_ok();

		if succeeds {

			let expected = Self::expect_from(iterator).unwrap();

			Ok(Some(expected))
			
		}
		
		else { Ok(None) }

	}

}
