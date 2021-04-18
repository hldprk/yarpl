
#[allow(deprecated)]
#[allow(const_item_mutation)]
#[allow(unused_must_use)]
#[cfg(test)]


mod tests {
	
	use yarpl::Parser;
	use yarpl::Many;
	use yarpl::Maybe;
	use yarpl::Must;
	use yarpl::Only;



	#[test]
	pub fn consume() {

		let mut parser = Parser::from("abc");

		parser.feed::<Only<"a">>();
		parser.feed::<Only<"b">>();
		parser.feed::<Only<"c">>();

		assert_eq!(parser.tokens().len(), 3);

	}

	#[test]
	pub fn many() {

		let mut parser = Parser::from("aaaa");

		parser.feed::<Many<Only<"a">, {0..5}>>();

		assert_eq!(parser.tokens().len(), 4);

	}

	#[test]
	pub fn maybe() {
		
		let parser = Parser::from("b");
		
		let a_result = parser.clone().feed::<Maybe<Only<"a">>>();
		let b_result = parser.clone().feed::<Maybe<Only<"b">>>();
		
		assert_eq!(a_result.is_ok(), b_result.is_ok());
		
	}
	
	#[test]
	pub fn must() {
		
		let parser = Parser::from("b");
	
		let result = parser.clone().feed::<Must<Only<"b">>>();
	
		assert!(result.is_ok());
		assert!(parser.top().is_none());
				
	}


}

