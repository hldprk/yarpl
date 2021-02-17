


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
    
    ($visibility:vis fn $name:ident { $($choices:ident ( ) ;)* } ) => {
    
        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {
            
            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            use std::rc::Rc;

            let name: &'static str = stringify!($name);

            let choices: Vec<fn(&str, usize) -> $crate::core::Result> = vec![ $($choices),* ];
            
            let mut valid_choices: Vec<Progress> = vec![];

            // Loops through `choices`, keeping all successful parses in 'valid_choices'.
            for choice in choices {

                if let Ok(progress) = choice(string, index) {

                    valid_choices.push( Progress::Nonempty {

                        offset: progress.offset(),

                        done: Done::Nonterminal {
                            
                            name,
                            
                            children: vec![Rc::from(progress.done().unwrap())]

                        }

                    });

                }

            }

            // Sorts successes by how many characters they parsed.
            valid_choices.sort_by(|a, b| a.offset().cmp(&b.offset()));

            // Returns the most successful choice
            if valid_choices.len() > 0 {
                
                let result = valid_choices[0].clone();

                Ok(result)

            } 
            
            // ... otherwise return 'Err(Fail)'.
            else {

                Err( Done::Fail { 
                
                    name,
                    done: None

                })

            }

        }

    };

}
