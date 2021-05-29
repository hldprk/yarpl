use core::num;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::*;

/// Returns a `String` of one or more decimal digit characters when parsed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number;

impl Expect for Number {

	type Target = String;

	fn expect_from(parser: &mut Parser) -> Result<Self::Target>
	where Self : Sized + Debug {
			
		let first_digits_maybe = Digits::expect_from(parser);

		let period_maybe = Just::<".">::expect_from(parser);

		let second_digits_maybe = Digits::expect_from(parser);

		let second_digits = if period_maybe.is_ok() & second_digits_maybe.is_ok() {

			second_digits_maybe.unwrap().to_string()

		} else {
			
			"0".to_string()
		
		}; 

		if first_digits_maybe.is_ok() {

			let first_digits = first_digits_maybe.unwrap();
			
			let number_string = first_digits.to_string() + "." + &second_digits.to_string();

			Ok(number_string)

		} else {

			Err(Unexpected::from(parser.clone()))

		}

		
	}

}
