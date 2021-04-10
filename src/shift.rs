
#[macro_export]

/// A macro to quickly define a type that will only shift a string when parsed.
///
/// # Example
///
/// ``` 
/// # use yarpl::shift;
/// # use yarpl::Consumer;
/// # use yarpl::Feed;
/// 
/// let ref mut consumer = Consumer::from("hello");
/// 
///	shift!(pub Hello "hello");
/// 
/// consumer.consume(&mut Hello::default());
///
/// assert_eq!(consumer.top().unwrap(), String::from("hello"));
///
/// ``` 
macro_rules! shift {
	
	($visibility:vis $Type:ident $string:expr) => {
		
		#[derive(Clone, Copy, Debug, Default)]
		$visibility struct $Type;

		impl $crate::Feed for $Type {

			fn feed(&mut self, consumer: &mut $crate::Consumer) -> $crate::Result {

				let string: &str = $string;

				consumer.shift(string)

			}

		}

	};

}