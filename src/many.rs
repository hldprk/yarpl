
use crate::feed::Feed;
use crate::consumer::Consumer;
use crate::feed::Result;
use std::ops::Range;
#[derive(Clone, PartialEq)]

/// Given a type `T : Feed` and a range `n .. m`, parses `T` between `n` and `m` times.
pub struct Many<T : Feed> {

	children : Vec<T>,
	range: Range<usize>

}

impl <T : Feed + Default + Clone> Feed for Many<T> {

	fn feed(&mut self, consumer: &mut Consumer) -> Result {
		
		let to_clone_from = Self::from(self.range.clone());

		let mut i = 0;

		loop {

			if i == self.range.end { break; }

			let ref mut to_push = T::default();

			let current_result = consumer.consume(to_push);

			if current_result.is_ok() { 
				
				self.children.push(to_push.clone()); 
			
				i += 1;

			}

			else { break; }

		}

		if self.range.contains(&i) {

			self.clone_from(&to_clone_from);

			Ok(())

		}
		
		else { Err(()) }

	}

}


impl<T : Feed> IntoIterator for Many<T> {

    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		
		self.children.into_iter()

	}

}

impl<T : Feed> From<Range<usize>> for Many<T> {

	fn from(range: Range<usize>) -> Self {
		
		Self {

			children: Vec::default(),
			range

		}

	}

} 