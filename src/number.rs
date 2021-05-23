use core::num;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::Digits;
use crate::Expect;
use crate::Just;

/// Parsed from an `Iterator` starting with a whole or decimal number.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(pub(crate) String);

impl Display for Number {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{}", self.0)

	}

}

impl Expect for Number {

	fn expect_from<I : Iterator<Item = char> + Clone>(iterator: &mut I) -> Result<Self, ()> where Self : Sized {
		
		let first_digits_maybe = Digits::expect_from(iterator);

		let period_maybe = Just::<".">::expect_from(iterator);

		let second_digits_maybe = Digits::expect_from(iterator);

		let second_digits = if period_maybe.is_ok() & second_digits_maybe.is_ok() {

			second_digits_maybe.unwrap().to_string()

		} else {
			
			"0".to_string()
		
		}; 

		if first_digits_maybe.is_ok() {

			let first_digits = first_digits_maybe.unwrap();
			
			let number_string = first_digits.to_string() + "." + &second_digits.to_string();

			Ok(Number(number_string))

		} else {

			Err(())

		}

		
	}

}
