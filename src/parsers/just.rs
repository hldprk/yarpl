

/// Maps a region of some string to a ```Done::Terminal```.
///
/// ## Matching a String
/// 
/// The most simple way to use this is by setting a function equal to some string literal inside `just!`.
///
/// ```
/// just!(pub fn number_one { "1" } );
///
/// assert!(number_one("1", 0).is_ok());
///
/// ```
///
/// ## Using a Function
/// 
/// Matching one specific string at a time isn't often ideal, so you may also include a function with the type ```fn(&char) -> bool```
/// which will iterate over the source string until the function returns `false.` If some characters matched, this `just!` function
/// will return [`Done::Terminal`], otherwise returns [`Done::Fail`].
///
/// ### Example 
/// ```
///
/// just!(pub fn ascii_characters { is_ascii(); } )
/// 
/// assert!(ascii_characters("abcde").is_ok())
///
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


    ($visibility:vis fn $name:ident { $predicate_function:ident(); }) => {
        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {

            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let predicate_function: fn(&char) -> bool = $predicate_function;
            
            let name = stringify!($name);
            
            let mut matched_string: String = string[index..].chars().take_while(|character: &char| predicate_function(character) ).collect();
            
            let mut offset = matched_string.len();

            if matched_string.len() == 0 {

                Err(Done::Fail {

                    name,
                    done: None

                })

            } 
            
            else {

                Ok(Progress::Nonempty {

                    offset,
                    done: Done::Terminal {

                        name,
                        matched_string

                    }

                })

            }

        }

    };


}
