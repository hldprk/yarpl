

/// Tries to match the provided string to the current place in the input.
///
/// ## Example
///
/// ```
/// just!( pub integer { "[0-9]+" });
/// ```
#[macro_export]
macro_rules! just {

    ( $visibility:vis fn $name:ident { $string_to_match:literal } ) => {

        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {
            
            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let name: &'static str = stringify!($name);
            let string_to_match: &'static str = $string_to_match;

            if string[index..].starts_with(string_to_match) {

                return Ok( Progress::Nonempty {

                    offset: string_to_match.len(),
                    
                    done: Done::Terminal {

                        name,

                        matched_string: string_to_match.to_owned()

                    }

                });

            } 
            
            Err( Done::Fail {
                
                name,
                done: None
            
            })

        }   

    };


}
