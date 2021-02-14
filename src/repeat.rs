
/// Using a modified, regular-expression-like syntax, repeatedly calls another parser the number of times specified.
///
/// ## Example
///
/// ```
/// just!(pub a { "a"; } );
/// 
/// repeat!(pub repeat_a_five_times { a; { 5 } } ); 
/// 
/// repeat!(pub repeat_a_at_least_five_times { a; { 5, } } ); 
///
/// repeat!(pub repeat_a_forever { a; { , } } ); 
///
/// ```
///
#[macro_export]
macro_rules! repeat {

    ($visibility:vis $name:ident { $to_repeat:ident; { $lower:expr , } } )=> { 

        repeat!($visibility $name { $to_repeat; { 1 , usize::MAX } } );

    };

    ($visibility:vis $name:ident { $to_repeat:ident; { , $upper:expr } } )=> { 

        repeat!($visibility $name { $to_repeat; { 0 , $upper }});

    };

    ($visibility:vis $name:ident { $to_repeat:ident; { , } } )=> { 

        repeat!($visibility $name { $to_repeat; { 0 , usize::MAX }});

    };

    ($visibility:vis $name:ident { $to_repeat:ident; { $amount:expr } } )=> { 

        repeat!($visibility $name { $to_repeat; { $amount , $amount }});

    };

    ($visibility:vis $name:ident { $to_repeat:ident; { $lower:expr , $upper:expr } } ) => { 

        $visibility fn $name ( string: String, index: usize ) -> $crate::parse_result::ParseResult {

            use $crate::Parser;
            use $crate::Progress;
            use $crate::ParseError;
            use $crate::Done;
            use std::rc::Rc;

            let lower: usize = $lower;
            let upper: usize = $upper;

            let to_repeat: Parser = $to_repeat;

            let name: &'static str = stringify!($name);

            let mut offset = 0;

            let mut children: Vec<Rc<Done>> = vec![];

            #[allow(unused_assignments)]
            let mut parse_error = ParseError {

                offset,
                
                name_stack: vec![],

                message: ""

            };
            
            loop {
                
                let result = to_repeat(string.clone(), index + offset);

                if let Ok(progress) = result {

                    children.push(Rc::from(progress.done));

                    offset += progress.offset;

                    continue;

                }

                if let Err(new_parse_error) = result {

                    parse_error = new_parse_error;

                    break;

                }

            }

            if children.len() < lower || children.len() > upper {
                
                let mut result = parse_error.clone();

                result.offset = offset;

                result.name_stack.push(name);

                result.message = concat!("Couldn't parse '", stringify!($name) ,"' enough times.");

                return Err(result.clone());

            }

            Ok( Progress { offset, done: Done::Nonterminal {
                
                name,

                children
            
            }})

        }

    };

}