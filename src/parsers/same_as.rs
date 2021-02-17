
/// An anonymous version of [`or!`] - renames its function's [`Result`] the name of its child parser.
#[macro_export]
macro_rules! same_as {

    ($visibility:vis fn $name:ident { $($other_name:ident();)+ } ) => {

        $visibility fn $name (string: &str, index: usize) -> $crate::core::Result {
            
            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            let name = stringify!($name);

            let other_names: Vec<&'static str> = vec![ $(stringify!($other_name),)+ ];
            let other_parsers: Vec<fn(&str, usize) -> Result> = vec![ $($other_name,)+ ];
            
            let mut valid_choices: Vec<Progress> = vec![]; 
            let mut invalid_choices: Vec<Done> = vec![]; 

            let mut i = 0;

            while i < other_names.len() {

                let other_parser = other_parsers[i];

                match other_parser( string.clone() , index ) {

                    Ok(progress) => {
                        
                        let mut done = progress.done().unwrap();

                        done.rename(name);

                        valid_choices.push(Progress::Nonempty { offset: progress.offset(), done } );

                    },

                    Err(done) => {

                        invalid_choices.push(done);

                    }

                }

                i += 1;

            }

            valid_choices.sort_by(|a, b| a.offset().cmp(&b.offset()));

            if valid_choices.len() < 1 {

                Err(invalid_choices.pop().unwrap())

            } 

            else {

                Ok(valid_choices.pop().unwrap())

            }

        }
    
    }

}