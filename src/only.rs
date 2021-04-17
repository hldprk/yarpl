

/// Defines a type that only parses a string.
///
/// # Example
///
/// ```
/// # use yarpl::only;
/// only!(A : "a");
/// ```
/// 
#[macro_export]
macro_rules! only {

	($Type:ident : $literal: literal) => {

		#[derive(Clone, Copy, Default, PartialEq, Debug)]
		pub struct $Type;

		impl $crate::Feed for $Type {

			fn feed(&mut self, consumer: &mut $crate::Consumer) -> $crate::Result {

				consumer.consume(&mut $literal)

			}

		}

	}

}