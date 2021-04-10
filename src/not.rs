use std::marker::PhantomData;

use crate::Consumer;
use crate::Feed;
use crate::Result;


#[derive(Clone, Debug, Default, Copy)]
/// A type that, when parsed, ensures an object of type `T : Feed` *cannot be parsed* from some `Consumer`.
pub struct Not<T : Feed>(PhantomData<T>);


impl<T : Feed + Default> Feed for Not<T> {

	fn feed(&mut self, consumer: &mut Consumer) -> Result {
		
		let ref mut consumer_cloned = consumer.clone();
		
		let result = consumer_cloned.consume(&mut T::default());

		match result {

			Ok(_) => Err(()),
			Err(_) => Ok(())

		}

	}

}