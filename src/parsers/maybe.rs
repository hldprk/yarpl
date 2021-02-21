
/// Will return `Ok(Progress)` whether or not its referenced parsing function is successful.
///
/// ### Example
/// ```
/// just!( fn space { " " } );
///
/// maybe!( fn maybe_space { " " } );
///
/// assert!( maybe_space(" ", 0).is_ok() );
/// assert!( maybe_space("asdfasfsa", 0).is_ok() );
///
///
/// ```
#[macro_export]
macro_rules! maybe {

    ($visibility:vis $name:ident { $other_name:ident; } ) => {

        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {
            
            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let other_parser: fn(&str, usize) -> $crate::core::Result = $other_name;

            match other_parser(string, index) {

                Ok(progress) => Ok(progress),

                Err(_) => Ok( Progress::Empty )

            }

        }

    };

}

