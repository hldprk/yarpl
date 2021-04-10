


#[macro_export]
/// A macro to define a type which solely calls `Consumer::shift_while` when parsed.
macro_rules! shift_while {
	
	($visibility:vis $Type:ident $function:expr) => {
		
		#[derive(Clone, Copy, Debug, Default)]
		$visibility struct $Type;

		impl $crate::Feed for $Type {

			fn feed(&mut self, consumer: &mut $crate::Consumer) -> $crate::Result {

				consumer.shift_while(&$function)

			}

		}

	};

}