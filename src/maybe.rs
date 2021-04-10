use crate::consumer;
use crate::feed::Feed;
use crate::consumer::Consumer;
use crate::feed::Result;

/// When parsed, always returns `Ok`. Will also construct a `Feed` object in `self` if successful.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Maybe<T : Feed> (Option<T>);

impl<T : Feed + Clone> Maybe<T> {

	/// Returns the inner parsed object as `Some(T)` if done and succesful, otherwise `None`.
	fn inner(&self) -> Option<T> {

		self.0.clone()

	}

}

impl<T : Feed + Default + Clone> Feed for Maybe<T> {

	fn feed(&mut self, consumer: &mut Consumer)	-> Result {

		let ref mut consumer_cloned = consumer.clone();
		
		let ref mut feed = T::default();

		let result = consumer_cloned.consume(feed);

		if result.is_ok() {

			consumer.clone_from(consumer_cloned);

			self.0 = Some(feed.clone());

			Ok(())

		}

		else { Ok(()) }

	}

}