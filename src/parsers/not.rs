

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

    ($visibility:vis fn $name:ident { $parser:ident(); }) => {
        
        $visibility fn $name(string: &str, index: usize) -> $crate::core::Result {

            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let name: &'static str = stringify!($name);

            // Returns 'Err(Done)' if the given parser *doesn't* fail.
            if let Ok(progress) = $parser( string, index ) {
                
                if let Progress::Empty = progress {

                    return Err( Done::Fail {

                        name,
                        done: None

                    });

                }

                if let Progress::Nonempty { done, .. } = progress {
    
                    return Err( Done::Fail { 
                            
                        name,
                        done: Some(Box::from(done))
                    
                    });

                }

            }   

            // ... otherwise returns an empty ( but successful ) Progress.
            Ok( Progress::Empty )
 
        }

    };

}