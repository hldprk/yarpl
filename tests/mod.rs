
#[allow(deprecated)]
#[allow(const_item_mutation)]
#[allow(unused_must_use)]
#[cfg(test)]


#[cfg(test)]
mod tests {

	use yarpl::*;

	#[test]
	fn expect_letters() {

		assert!(Letters::expect_from(&mut Parser::from("asdf")).is_ok());

	}

	#[test]
	fn expect_digits() {

		assert!(Digits::expect_from(&mut Parser::from("123asdf")).is_ok());

	}

	#[test]
	fn expect_space() {

		assert!(Spaces::expect_from(&mut Parser::from("\t\n  ")).is_ok());

	}

	#[test]
	fn expect_just() {

		assert!(Just::<"yeah">::expect_from(&mut Parser::from("yeahnah")).is_ok());

	}

	#[test]
	fn expect_maybe() {
		
		//assert!(Just::<"yeah">::maybe_expect_from(&mut Parser::from("asdf")).is_ok());
		//assert!(Just::<"yeah">::maybe_expect_from(&mut Parser::from("yeah")).is_ok());

	}

	#[test]
	fn expect_number() {

		assert!(Number::expect_from(&mut Parser::from("1234.5678")).is_ok());
		
	}

	#[test]
	fn expect_type_alias() {
		
		type Newline = Just::<"\n">;
		
		assert!(Parser::from("\n").expect::<Newline>().is_ok());
		
	}
	
	#[test]
	fn expect_error() {

		let ref mut parser = Parser::from("abc");

		let _ = parser.expect::<Just<"a">>();
		let _ = parser.expect::<Just<"b">>();
		let result = parser.expect::<Just<"d">>();

		println!("{}", result.unwrap_err())

	}

	
}

