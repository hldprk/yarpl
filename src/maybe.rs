
/// Will return an `Ok(Progress)` whether or not its held parsing function is successful.
#[macro_export]
macro_rules! maybe {

    ($visibility:vis $name:ident { $other_name:ident; } ) => {

        $visibility fn $name (string: String, index: usize) -> Result<$crate::progress::Progress, $crate::parse_error::ParseError> {
            
            use $crate::Parser;
            use $crate::Progress;
            use $crate::Done;

            let other_parser: Parser = $other_name;

            match other_parser(string.clone(), index) {

                Ok(progress) => Ok(progress),

                Err(_) => Ok( Progress {

                    offset: 0,

                    done: Done::Empty()

                })

            }

        }

    };

}

