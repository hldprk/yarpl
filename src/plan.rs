
/// Defines a type which parses a sequence of predefined `Feed` types.
///
/// # Example
///
/// ```
/// # use yarpl::plan;
/// # use yarpl::only;
/// only!(A : "a");
///
/// only!(B : "b");
///
/// only!(C : "c");
///
/// plan!(ABC : A, B, C);
/// ```
/// 
#[macro_export]
macro_rules! plan {

	($Type:ident : $($Types:ident),* $(,)?) => {

		#[derive(Clone, Copy, Default, PartialEq, Debug)]
		pub struct $Type;

		impl $crate::Feed for $Type {

			fn feed(&mut self, consumer: &mut $crate::Consumer) -> $crate::Result {

				$(consumer.consume(&mut $Types::default())?;)*

				Ok(())

			}

		}

	}

}