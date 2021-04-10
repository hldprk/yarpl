use std::marker::PhantomData;

use crate::Consumer;
use crate::Feed;
use crate::Result;

#[derive(Clone, Debug, Default, Copy)]
/// When parsed, ensures an object of type `T : Feed` *is able* to be parsed from a consumer, but will consume no input.
pub struct Must<T : Feed + Default>(PhantomData<T>);


impl<T : Feed + Default> Feed for Must<T> {

	fn feed(&mut self, consumer: &mut Consumer) -> Result {
		
		let ref mut consumer_cloned = consumer.clone();
		
		consumer_cloned.consume(&mut T::default())

	}

}