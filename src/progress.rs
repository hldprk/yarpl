

use std::fmt::Display;
use std::fmt::Formatter;

use crate::done::Done;

/// Used to track progress of an on-going parse.
#[derive(Debug, Clone)]
pub enum Progress {

    Empty,

    Nonempty {
        
        /// How many characters a parse offset the index from its previous position in the input string.    
        offset: usize,
    
        /// The [``Done``] returned from some parse.
        done: Done, 

    }

}

impl Progress {

    pub fn offset (&self) -> usize {

        if let Progress::Nonempty { offset, .. } = self {

            return *offset;

        }

        0

    }    
    
    pub fn done (&self) -> Option<Done> {

        if let Progress::Nonempty { done, .. } = self {

            return Some(done.clone());

        }

        None

    }

}

impl Display for Progress {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        write!(formatter, "{} \n\n ...progressed {} characters.", self.done().unwrap(), self.offset() );
        
        Ok(())

    }

}