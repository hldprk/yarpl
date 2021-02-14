

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
    
    ($visibility:vis $name:ident { $($identifiers:ident;)* } ) => {
    
        $visibility fn $name (string: String, index: usize) -> $crate::parse_result::ParseResult {

            use std::rc::Rc;
            use $crate::Done;
            use $crate::Parser;
            use $crate::Progress;

            let name: &'static str = stringify!($name);

            let sequence: Vec<Parser> = vec![$($identifiers),*];

            let mut offset: usize = 0;
            
            let mut children: Vec<Rc<Done>> = vec![];

            for parser in sequence {
                
                let progress_maybe = parser(string.clone(), index + offset);

                match progress_maybe {
                    
                    Ok(progress) => {

                        offset += progress.offset;

                        children.push(Rc::from(progress.done));

                    }

                    Err(parse_error) => {

                        let mut new_parse_error = parse_error;
                        
                        new_parse_error.name_stack.push(name);

                        return Err(new_parse_error);

                    }

                }

            }

            Ok( Progress {

                offset,
                
                done: Done::Nonterminal {

                    name,

                    children
                    
                }
            
            })
 
        }

    }
}

