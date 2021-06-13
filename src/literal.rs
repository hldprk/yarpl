use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;

use crate::*;

/// Matches a `&'static str`, and returns the string if successful. 
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Literal<const LITERAL: &'static str>;

impl<const LITERAL: &'static str> Parse for Literal<LITERAL> {

	type Target = &'static str;

	fn parse_from(parser: &mut Parser) -> Result<Self::Target> {
			
		let remainder: String = parser.iterator.clone().collect();

		if remainder.starts_with(LITERAL) {

			parser.iterator.advance_by(LITERAL.len());

			Ok(LITERAL)

		} 
		
		else { Err(Error::new::<Self>(parser)) }
		
	}

}
