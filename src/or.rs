


/// Holds the result of the most successful symbol it parses, if there is one.
///
/// ### Example
/// ```
/// just!( plus { "+"; } );
/// 
/// just!( minus { "-"; } );
/// 
/// or!( plus_or_minus { plus; minus; } );
///
/// ```
#[macro_export]
macro_rules! or {
    
    ($visibility:vis $name:ident { $($choices:ident;)* } ) => {
    
        $visibility fn $name (string: String, index: usize) -> $crate::ParseResult {
            
            use $crate::Parser;
            use $crate::Progress;
            use $crate::Done;
            use $crate::ParseError;
            use std::rc::Rc;

            let name: &'static str = stringify!($name);

            let choices: Vec<Parser> = vec![ $($choices),* ];
            
            let mut valid_choices: Vec<Progress> = vec![];

            // Loops through `choices`, keeping all successful parses in 'valid_choices'.
            for choice in choices {

                if let Ok(progress) = choice(string.clone(), index) {

                    valid_choices.push( Progress {

                        offset: progress.offset,

                        done: Done::Nonterminal {
                            
                            name,
                            
                            children: vec![Rc::from(progress.done)]

                        }

                    });

                }

            }

            // Sorts successes by how many characters they parsed.
            valid_choices.sort_by(|a, b| a.offset.cmp(&b.offset));

            // Returns the first choice if it exists ...
            if valid_choices.len() > 0 {
                
                let result = valid_choices[0].clone();

                Ok(result)

            } 
            
            // ... otherwise return 'Err(ParseError)'.
            else {

                Err( ParseError {

                    offset: 0,

                    name_stack: vec![name],

                    message: concat!("No valid symbol found in '", stringify!($name), "'.")

                })

            }

        }

    };

}
