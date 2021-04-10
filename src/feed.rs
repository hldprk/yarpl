
use crate::consumer::Consumer;

pub type Result = std::result::Result<(), ()>;

/// Defines how a type may be parsed by mutating `self` and some `Consumer`.
pub trait Feed {

	fn feed(&mut self, consumer: &mut Consumer) -> Result;

}

