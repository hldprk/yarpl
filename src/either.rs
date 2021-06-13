use std::marker::PhantomData;
use std::fmt::Debug;

use crate::*;

#[derive(Clone, Debug, Copy, PartialEq)]

/// Matches one of the characters included in the `&'static str` provided, then returns that character.
pub struct Either<const STRING: &'static str>;

impl<const STRING: &'static str> Parse for Either<STRING> {

	type Target = char;

	fn parse_from(parser: &mut Parser) -> Result<Self::Target> {
		
		let next_character_maybe = parser.iterator.next();

		if next_character_maybe.is_none() {

			Err(Error::new::<Self>(parser))

		} else {

			let next_character = next_character_maybe.unwrap();

			if STRING.contains(next_character) { Ok(next_character) }

			else { Err(Error::new::<Self>(parser)) }

		}

	}

}