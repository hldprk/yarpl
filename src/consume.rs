
use crate::Parser;

pub type Result<T> = std::result::Result<T, ()>;

/// Returns `Self::Target` when parsed by `Parser::feed`.
pub trait Consume {

	type Target;

	fn consume(parser: &mut Parser) -> Result<Self::Target>;

}

