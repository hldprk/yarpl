
/// Will return an `Ok(Progress)` whether or not its held parsing function is successful.
#[macro_export]
macro_rules! maybe {

    ($visibility:vis $name:ident { $other_name:ident; } ) => {

        $visibility fn $name (string: String, index: usize) -> $crate::ParseResult {
            
            use $crate::ParseResult;
            use $crate::Progress;
            use $crate::Done;

            let other_parser: fn(String, usize) -> ParseResult = $other_name;

            match other_parser(string.clone(), index) {

                Ok(progress) => Ok(progress),

                Err(_) => Ok( Progress::Empty )

            }

        }

    };

}

