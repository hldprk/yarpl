
#[allow(deprecated)]
#[allow(const_item_mutation)]
#[allow(unused_must_use)]
#[cfg(test)]


#[cfg(test)]
mod tests {

	use yarpl::Expect;
	use yarpl::Digits;
	use yarpl::Just;
	use yarpl::Letters;
	use yarpl::Number;
	use yarpl::Spaces;

	#[test]
	fn expect_letters() {

		let _ = Letters::expect_from(&mut "asdf".chars());

	}

	#[test]
	fn expect_digits() {

		let _ = Digits::expect_from(&mut "123asdf".chars());

	}

	#[test]
	fn expect_space() {

		let _ = Spaces::expect_from(&mut "\t\n  ".chars());

	}

	#[test]
	fn expect_just() {

		let _ = Just::<"yeah">::expect_from(&mut "yeahnah".chars());

	}

	#[test]
	fn expect_maybe() {
		
		assert!(Just::<"yeah">::expect_from_maybe(&mut "asdf".chars()).is_ok());
		assert!(Just::<"yeah">::expect_from_maybe(&mut "yeah".chars()).is_ok());

	}

	#[test]
	fn expect_number() {

		assert!(Number::expect_from(&mut "1234.5678".chars()).is_ok());
		
	}
	
	#[test]
	fn expect_type_alias() {
		
		type Newline = Just::<"\n">;

		assert!(Newline::expect_from(&mut "\n".chars()).is_ok());

	}

	
}

