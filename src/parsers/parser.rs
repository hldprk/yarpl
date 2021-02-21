
struct VirtualAttribute<T> {

	name: String,
	argument: Option<T>

}


#[macro_export]
macro_rules! parser {

	(

		$(#[ $attribute_name:ident $(= $attribute_argument:ident )* ] )*
		$visibility:vis fn $name:ident {

			$($function:ident();)*

		}

	) => {

		/*

		$visibility fn $name ( string: &str, index: usize ) -> $crate::core::Result {

			use $crate::core::Progress;
			use $crate::core::Done;
			use std::rc::Rc;

			let virtual_attribute_names: Vec<&'static str> = vec![$(stringify!($attribute_name),)*];
			
			let functions: Vec<fn(&str, usize) -> $crate::core::Result> = vec![$($function,)*];

			let has_attribute = |attribute: &'static str| 
					virtual_attribute_names.iter().find(
						|other_attribute| 
							attribute == **other_attribute
					).is_some();

			let same_as: bool = has_attribute("anonymous");
			let or: bool = has_attribute("or") || same_as;
			let repeat: bool = has_attribute("repeat");
			let maybe: bool = has_attribute("maybe");
			let not: bool = has_attribute("not");

			let mut results: Vec<$crate::core::Result> = vec![];

			let mut offset = 0;

			for function in functions {

				// The offset is applied to the current index if this isn't 'or'.
				let mut index_to_use = if or { index } else { index + offset };

				let result = function(string, index_to_use);
				
				if result.is_err() && (!or) {

					// A result of 'Err(_)' doesn't stop parsing if this parser is 'or'.

					break;

				} 
				
				else {
					
					results.push(result);

				}

			}

			let mut result: $crate::core::Result;

			if results.len() < 1{


			}

			Ok(Progress::Empty)

		} 
		*/

	}

}
