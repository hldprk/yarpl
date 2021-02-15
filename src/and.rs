

/// Calls a sequence of parsers, failing completely if one returns `Err`.
///
/// ## Example 
/// ```
/// just!( left_number { "123"; } );
/// just!( right_number { "456"; } );
/// just!( plus { "+"; } );
///
/// and!( addition { 
///
///     left_number;
///     plus; 
///     right_number; 
///
/// });
///
/// assert!( addition(String::from("123+456"), 0).is_ok() );
/// ```
///
#[macro_export]
macro_rules! and {
    
    ($visibility:vis $name:ident { $($function:ident ( ) ;)* } ) => {
    
        $visibility fn $name (string: String, index: usize) -> $crate::ParseResult {

            use std::rc::Rc;
            use $crate::Done;
            use $crate::ParseResult;
            use $crate::Progress;

            let name: &'static str = stringify!($name);

            let sequence: Vec<fn(String, usize) -> ParseResult> = vec![$($function),*];

            let mut offset: usize = 0;
            
            let mut children: Vec<Rc<Done>> = vec![];

            for parser in sequence {
                
                let progress_maybe = parser(string.clone(), index + offset);

                match progress_maybe {
                    
                    Ok(progress) => {

                        offset += progress.offset();

                        children.push(Rc::from(progress.done().unwrap()));

                    }

                    Err(parse_error) => {

                        let mut new_parse_error = parse_error;
                        
                        new_parse_error.name_stack.push(name);

                        return Err(new_parse_error);

                    }

                }

            }

            Ok( Progress::Nonempty {

                offset,
                
                done: Done::Nonterminal {

                    name,

                    children
                    
                }
            
            })
 
        }

    }
}

