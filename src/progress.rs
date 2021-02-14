

use std::fmt::Display;
use std::fmt::Formatter;

use crate::done::Done;

/// Used to track progress of an on-going parse.
#[derive(Debug, Clone)]
pub struct Progress {

    /// How many characters a parse offset the index from its previous position in the input string.    
    pub offset: usize,

    /// The [``Done``] returned from some parse.
    pub done: Done, 

}


impl Display for Progress {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        write!(formatter, "{} \n\n ...progressed {} characters.", self.done, self.offset );
        
        Ok(())

    }

}