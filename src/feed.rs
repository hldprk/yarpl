
use crate::consumer::Consumer;

pub type Result = std::result::Result<(), ()>;

/// Defines how a type may be parsed by mutating `self` and some `Consumer`.
pub trait Feed {

	fn feed(&mut self, consumer: &mut Consumer) -> Result;

}

impl Feed for &str {

	fn feed(&mut self, consumer: &mut Consumer) -> Result {

		let string = String::from(*self);

		if consumer.remainder.starts_with(&string) {
			
			consumer.remainder = consumer.remainder.split_at(self.len()).1.to_string();
			
			consumer.tokens.push(self.to_string());
			
			Ok(())
		}
		
		else { Err(()) }
		
	}

}




