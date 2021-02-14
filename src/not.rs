

///  Will return the opposite result of some other parser.
///
/// ## Example
///
/// ```
/// just!( pub plus { "+"; } );
///
/// not!( pub not_a_plus { plus; } );
/// ```
///
#[macro_export]
macro_rules! not {

    ($visibility:vis $name:ident { $parser:ident; }) => {
        
        $visibility fn $name(string: String, index: usize) -> $crate::parse_result::ParseResult {

            use $crate::progress::Progress;
            use $crate::parse_error::ParseError;
            use $crate::done::Done;

            let name: &'static str = stringify!($name);

            // Returns a 'Err(ParseError)' if the given parser doesn't fail.
            if let Ok(_) = $parser( string, index ) {
                
                let parse_error = ParseError {

                    offset:0,
                    
                    name_stack: vec![name],
                    
                    message: concat!("'", stringify!($name), "' not allowed.")
                };

                return Err(parse_error);

            }   

            // ... otherwise returns an empty ( but successful ) Progress.
            Ok( Progress { offset: 0, done: Done::Empty() })
 
        }

    };

}