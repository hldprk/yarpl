use crate::Feed;
use crate::Consumer;
use crate::Result;

#[derive(Clone, Debug)]
/// Consumes a single token built from a `char` predicate.
pub struct Peek<F : Fn(char) -> bool>(F);


impl<F : Fn(char) -> bool> Peek<F> {
    
	pub fn new(function: F) -> Self {
		
		Self(function)

	}

}

impl<F : Fn(char) -> bool> Feed for Peek<F> {

	fn feed(&mut self, consumer: &mut Consumer) -> Result {
		
		let string: String = 
			consumer.remainder().chars().take_while(
				|character|
					self.0(*character)).collect();

		let ref mut str = string.as_str();

		consumer.consume(str)
		
	}

}

/// Defines a type that solely parses with `Peek`.
///
/// # Example
///
/// ```
/// # use yarpl::peek;
/// peek!(Letters : char::is_alphabetic);
/// ```
/// 
#[macro_export]
macro_rules! peek {

	($Type:ident : $peeker: expr) => {

		#[derive(Clone, Copy, Default, PartialEq, Debug)]
		pub struct $Type;

		impl $crate::Feed for $Type {

			fn feed(&mut self, consumer: &mut $crate::Consumer) -> $crate::Result {

				let ref mut peeker = $crate::Peek::new($peeker);
				
				consumer.consume(peeker)?;

				Ok(())

			}

		}

	}

}