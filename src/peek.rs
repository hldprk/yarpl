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

		consumer.consume_str(&string)
		
	}

}