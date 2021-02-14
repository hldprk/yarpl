use crate::parse_result::ParseResult;
use crate::parser::Parser;

/// An anonymous version of [`or!`] - renames its function's [`ParseResult`] the name of its child [`Parser`].
#[macro_export]
macro_rules! same_as {

    ($visibility:vis $name:ident { $($other_name:ident;)+ } ) => {

        $visibility fn $name (string: String, index: usize) -> Result<$crate::progress::Progress, $crate::parse_error::ParseError> {
            
            use $crate::progress::Progress;
            use $crate::parse_error::ParseError;
            use $crate::parser::Parser;

            let name = stringify!($name);

            let other_names: Vec<&'static str> = vec![ $(stringify!($other_name),)+ ];
            let other_parsers: Vec<Parser> = vec![ $($other_name,)+ ];
            
            let mut valid_choices: Vec<Progress> = vec![]; 
            let mut invalid_choices: Vec<ParseError> = vec![]; 

            let mut i = 0;

            while i < other_names.len() {

                let other_parser = other_parsers[i];

                match other_parser( string.clone() , index ) {

                    Ok(progress) => {
                        
                        let mut done = progress.done;

                        done.rename(name);

                        valid_choices.push(Progress { offset: progress.offset, done } );

                    }

                    Err(mut error) => {

                        if let Some(_) = error.name_stack.pop() {

                            error.name_stack.push(name);

                        }
                        
                        invalid_choices.push(error)

                    }

                }

                i += 1;

            }

            valid_choices.sort_by(|a, b| a.offset.cmp(&b.offset));
            invalid_choices.sort_by(|a, b| a.offset.cmp(&b.offset));

            if valid_choices.len() < 1 {

                Err(invalid_choices.pop().unwrap())

            } 

            else {

                Ok(valid_choices.pop().unwrap())

            }

        }
    
    }

}