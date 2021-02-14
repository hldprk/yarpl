

/// Tries to match the provided string to the current place in the input.
///
/// ## Example
///
/// ```
/// just!( pub integer { "[0-9]+"; });
/// ```
#[macro_export]
macro_rules! just {

    ( $visibility:vis $name:ident { $string_to_match:literal; } ) => {

        $visibility fn $name (string: String, index: usize) -> $crate::ParseResult {
            
            use $crate::Done;
            use $crate::Progress;
            use $crate::ParseError;

            let name: &'static str = stringify!($name);
            let string_to_match: &'static str = $string_to_match;

            if string[index..].starts_with(string_to_match) {

                return Ok( Progress {

                    offset: string_to_match.len(),
                    
                    done: Done::Terminal {

                        name,

                        matched_string: string_to_match.to_owned()

                    }

                });

            } 
            
            Err( ParseError {

                offset: 0,
                name_stack: vec![ name ],
                message: concat!("Expected \"", $string_to_match, "\".")

            })

        }   

    };


}
