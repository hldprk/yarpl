
#[allow(deprecated)]

#[cfg(test)]
mod tests {
	
	use yarpl::Consumer;
	use yarpl::Result;
	use yarpl::Must;
	use yarpl::Not;
	use yarpl::Many;
	use yarpl::Maybe;

	yarpl::only!(A : "a");
	yarpl::only!(B : "b");
	yarpl::only!(C : "c");

	yarpl::plan!(ABC : A, B, C);

	yarpl::peek!(D : |character| character == 'd');

	#[test]
	pub fn consume() -> Result {

		let ref mut consumer = Consumer::from("abc");

		consumer.consume(&mut "a")?;
		consumer.consume(&mut "b")?;
		consumer.consume(&mut "c")?;

		assert_eq!(consumer.tokens().len(), 3);

		Ok(())

	}

	#[test]
	pub fn must() -> Result {

		let ref mut consumer = Consumer::from("abc");
		
		let result  = consumer.consume(&mut Must::<ABC>::default());
		
		assert!(consumer.tokens().is_empty());

		result

	}

	#[test]
	pub fn not() {

		let ref mut consumer = Consumer::from("a");
		
		let result  = consumer.consume(&mut Not::<A>::default());
		
		assert!(consumer.tokens().is_empty());

		assert!(result.is_err());

	}

	#[test]
	pub fn many() {

		let ref mut consumer = Consumer::from("abcabcabc");

		let result = consumer.consume(&mut Many::<ABC>::from(1..4));

		assert!(consumer.tokens().len() == 9);
		println!("{:?}", consumer.tokens());

		assert!(result.is_ok());

	}

	#[test]
	pub fn maybe() -> yarpl::Result {

		let ref mut consumer = Consumer::from("ab");

		let ref mut maybe_a = Maybe::<A>::default();

		consumer.consume(maybe_a)?;
		consumer.consume(maybe_a)

	}

	#[test]
	pub fn peek() -> yarpl::Result {

		let ref mut consumer = Consumer::from("dddd");

		let ref mut d = D::default();
		
		consumer.consume(d)?;

		assert!(consumer.top().unwrap().len() == 4);

		Ok(())

	}

}

