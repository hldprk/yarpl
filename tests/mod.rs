
#[allow(deprecated)]
#[allow(const_item_mutation)]
#[allow(unused_must_use)]
#[cfg(test)]


#[cfg(test)]
mod tests {

	use yarpl::*;

	type Letters = Many<Either<"abcdefghijklmnopqrstuvwxyz">>;
	type Digits = Many<Either<"0123456789">>;

	#[test]
	fn parse_letters() {

		let ref mut parser = Parser::from("asdf");

		assert!(parser.parse::<Letters>().is_ok());

	}

	#[test]
	fn parse_digits() {

		let ref mut parser = Parser::from("1234asdf");

		assert!(parser.parse::<Digits>().is_ok());
	}


	#[test]
	fn parse_literal() {
		
		type AAAA = Literal<"aaaa">;

		let ref mut parser = Parser::from("asdf    \naaaa");

		parser.disable_strict_mode();

		let result = parser.parse::<Literal<"safaffsda">>();
		let _ = parser.parse::<AAAA>();


		if result.is_err() {

			println!("{}", result.unwrap_err())

		}

	}

	#[test]
	fn parse_maybe() {
		
		let ref mut parser = Parser::from("asdf");
		let ref mut other_parser = parser.clone();

		assert!(parser.parse::<Maybe::<Literal<"asdf">>>().is_ok());
		assert!(other_parser.parse::<Maybe::<Literal<"yeah">>>().is_ok());

	}


	#[test]
	fn parse_type_alias() {
		
		type Newline = Literal::<"\n">;
		
		assert!(Parser::from("\n").parse::<Newline>().is_ok());
		
	}
	
	#[test]
	fn parse_either() {
		
		type Yeah = Either<"0123456789">;
		
		let ref mut parser = Parser::from("823813");
	
		assert!(parser.parse::<Yeah>().is_ok());
		
	} 
	
	
}

