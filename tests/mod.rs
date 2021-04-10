
#[allow(deprecated)]

#[cfg(test)]
mod tests {
	
	use yarpl::Consumer;
	use yarpl::Result;
	use yarpl::Feed;
	use yarpl::Must;
	use yarpl::Not;
	use yarpl::Many;
	use yarpl::Maybe;
	use yarpl::shift;
    use yarpl::shift_while;

	#[derive(Clone, Default, Copy)]
	pub struct A;	

	impl Feed for A {

		fn feed(&mut self, consumer: &mut Consumer) -> Result {
			
			consumer.shift("a")

		}

	}

	#[test]
	pub fn shift_without_macro() -> Result {

		let ref mut consumer = Consumer::from("abc1234");

		consumer.shift("a")?;
		consumer.shift("b")?;
		consumer.shift("c")?;
		consumer.shift_while(&|character: char| "1234567890".contains(character))

	}

	shift!(pub B "b");

	shift_while!(pub C &|character| character == 'c');

	#[test]
	pub fn shift_macro() -> Result {

		let ref mut consumer = Consumer::from("bcccc");

		consumer.consume(&mut B::default())?;
		consumer.consume(&mut C::default())?;

		assert_eq!(consumer.taken().len(), 2);

		Ok(())
		
	}
	
	#[test]
	pub fn must() -> Result {

		let ref mut consumer = Consumer::from("a");
		
		let result  = consumer.consume(&mut Must::<A>::default());
		
		assert!(consumer.taken().is_empty());

		result

	}

	#[test]
	pub fn not() {

		let ref mut consumer = Consumer::from("a");
		
		let result  = consumer.consume(&mut Not::<A>::default());
		
		assert!(consumer.taken().is_empty());

		assert!(result.is_err());

	}

	#[test]
	pub fn many() {

		let ref mut consumer = Consumer::from("aaa");

		let result = consumer.consume(&mut Many::<A>::from(1..4));

		assert!(consumer.taken().len() == 3);

		assert!(result.is_ok());

	}

	#[test]
	pub fn maybe() -> yarpl::Result {

		let ref mut consumer = Consumer::from("ab");

		let ref mut maybe_a = Maybe::<A>::default();

		consumer.consume(maybe_a)?;
		consumer.consume(maybe_a)

	}

}

