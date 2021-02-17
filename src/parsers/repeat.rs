
/// Repeatedly calls another parser the number of times specified.
///
/// ## Example
/// ```
///
/// just!(pub fn a { "a"; } );
/// 
/// repeat!(pub fn repeat_a_five_times { a(5, 5); } ); 
/// 
/// repeat!(pub fn repeat_a_at_least_five_times { a( 5, usize::MAX ); } ); 
///
/// repeat!(pub fn repeat_a_forever { a( 1, usize::MAX ); } ); 
///
///
/// ```
///
#[macro_export]
macro_rules! repeat {

    ($visibility:vis fn $name:ident { $to_repeat:ident ( $lower:expr , $upper:expr ); } ) => { 

        $visibility fn $name ( string: &str, index: usize ) -> $crate::core::Result {

            use $crate::core::Done;
            use $crate::core::Result;
            use $crate::core::Progress;

            use std::rc::Rc;

            let lower: usize = $lower;
            let upper: usize = $upper;

            let to_repeat: fn(&str, usize) -> Result = $to_repeat;

            let name: &'static str = stringify!($name);

            let mut offset = 0;

            let mut children: Vec<Rc<Done>> = vec![];
            
            loop {
                
                let result = to_repeat(string.clone(), index + offset);

                if let Ok(progress) = result {

                    children.push(Rc::from(progress.done().unwrap()));

                    offset += progress.offset();

                    continue;

                }

                if result.is_err() {

                    break;

                }
            
            }

            if children.len() < lower || children.len() > upper {
                
                return Err(Done::Fail { 
                
                    name,
                    done: None 
                
                });

            }

            Ok( Progress::Nonempty { 
                
                offset,
                
                done: Done::Nonterminal {
                    
                    name,

                    children
                
                }
            
            })

        }

    };

}