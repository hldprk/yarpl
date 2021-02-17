

/// Calls a sequence of parsers, failing completely if one returns `Err`.
///
/// ## Example 
/// ```
/// just!( fn left_number { "123" } );
/// just!( fn right_number { "456" } );
/// just!( fn plus { "+" } );
///
/// and!( fn addition { 
///
///     left_number();
///     plus(); 
///     right_number(); 
///
/// });
///
/// assert!( addition(String::from("123+456"), 0).is_ok() );
/// ```
///
#[macro_export]
macro_rules! and {
    
    ($visibility:vis fn $name:ident { $($function:ident() ;)* } ) => {
    
        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {

            use std::rc::Rc;
            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let name: &'static str = stringify!($name);

            let sequence: Vec<fn(&str, usize) -> $crate::core::Result> = vec![$($function),*];

            let mut offset: usize = 0;
            
            let mut children: Vec<Rc<Done>> = vec![];

            for parser in sequence {
                
                let progress_maybe = parser(string, index + offset);

                match progress_maybe {
                    
                    Ok(progress) => {

                        offset += progress.offset();

                        children.push(Rc::from(progress.done().unwrap()));

                    }

                    Err(done) => return Err( Done::Fail { 
                        
                        name,
                        done: Some(Box::from(done)) 
                    
                    })

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

