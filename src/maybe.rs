use crate::consume::Consume;
use crate::Parser;
use crate::Result;

/// When parsed, always returns `Ok`. Will also construct a `Consume` object in `self` if successful.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Maybe<T : Consume> (Option<T>);

impl<T : Consume + Clone> Maybe<T> {

	/// Returns the inner parsed object as `Some(T)` if done and succesful, otherwise `None`.
	fn inner(&self) -> Option<T> {

		self.0.clone()

	}

}

impl<T : Consume> Consume for Maybe<T> {

	type Target = Option<T::Target>;

	fn consume(parser: &mut Parser) -> Result<Self::Target> {

		let mut parser_cloned = parser.clone();

		let result = parser_cloned.feed::<T>();

		if result.is_ok() {

			parser.clone_from(&parser_cloned);

			Ok(Some(result.unwrap()))
			
		}

		else { Ok(None) }

	}

}