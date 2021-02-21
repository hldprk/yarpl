

use std::fmt::Display;
use std::fmt::Formatter;

use super::Done;

/// Used to track progress of an on-going parse.
#[derive(Debug, Clone)]
pub enum Progress {

    /// For when a parse didn't fail, but also didn't yield a [`Done`].
    Empty,

    /// Holds a [`Done`] and the offset relative to the index passed to the parent parser.
    Nonempty {
        
        offset: usize,
        done: Done, 

    }

}

impl Progress {

    /// Returns the number of characters offset relative to the starting index of the parent parser.
    ///
    /// Returns `0` if `self.is_empty()`.
    pub fn offset (&self) -> usize {

        if let Progress::Nonempty { offset, .. } = self {

            return *offset;

        }

        0

    }    
    
    /// Returns `Some(Done)` if `self.is_nonempty()`, otherwise `None`.
    pub fn done (&self) -> Option<Done> {

        if let Progress::Nonempty { done, .. } = self {

            return Some(done.clone());

        }

        None

    }

    /// Returns whether this `Progress` is `Empty`.
    pub fn is_empty(&self) -> bool {

        if let Progress::Empty = self {

            return true;

        }

        false

    }
    
    /// Returns whether this `Progress` is `Nonempty`.
    pub fn is_nonempty(&self) -> bool {

        if let Progress::Empty = self {

            return true;

        }

        false

    }

}

impl Display for Progress {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        if self.is_empty() {
        
            writeln!(formatter, "\nNo progress was made.");

        } else {

            writeln!(formatter, "{}\n...progressed {} characters.\n", self.done().unwrap(), self.offset() );

        }
        
        Ok(())

    }

}