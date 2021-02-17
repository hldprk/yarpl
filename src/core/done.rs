

use std::rc::Rc;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Concrete syntax tree returned from a parser.
#[derive(Debug, Clone)]
pub enum Done {

    /// Holds the name of the current failed branch and the next unsuccessful child node if there is one.
    Fail {

        name: &'static str,
        done: Option<Box<Done>>

    },

    /// A leaf node - holds the match from the original string as well as it's name.
    Terminal {

        name: &'static str,
        matched_string: String

    },

    /// A nonterminal node that has a name and branches.
    Nonterminal {

        name: &'static str,
        children: Vec<Rc<Self>>

    },


}


impl Done {

    /// Returns this name.
    pub fn name (&self) -> &'static str {

        match self {

            Done::Fail { name, ..} => name,

            Done::Terminal { name,.. } => name,

            Done::Nonterminal { name, ..} => name,

        }

    }
    
    /// Returns children of this [`Done`] if it has any.
    pub fn children (&self) -> Option<Vec<Rc<Done>>> {

        if let Done::Nonterminal { children, .. } = self {

            return Some(children.clone());

        }

        None        

    }

    /// Returns the matched string of this [`Done`] if it has one.
    pub fn matched_string (&self) -> Option<String> {

        if let Done::Terminal { matched_string, .. } = self {

            return Some(matched_string.clone());

        }

        None        

    }

    /// Returns whether this node is [`Done::Terminal`].
    pub fn is_terminal(&self) -> bool {

        if let Self::Terminal {..} = self {

            return true;

        }

        false

    }
    /// Returns whether this node is [`Done::Nonterminal`].
    pub fn is_nonterminal(&self) -> bool {

        if let Self::Nonterminal {..} = self {

            return true;

        }

        false

    }
    
    /// Returns whether this node is `Done::Fail`.
    pub fn is_fail(&self) -> bool {

        if let Self::Fail {..} = self {

            return true;

        }

        false

    }

    pub fn rename(&mut self, new_name: &'static str) -> () {

        if let Self::Nonterminal {ref mut name, ..} = self {

            *name = new_name;

        }
        
        if let Self::Terminal {ref mut name, ..} = self {

            *name = new_name;

        }

    }

}

// This is just help with with `Display`
static INDENTATION_COUNT: AtomicUsize = AtomicUsize::new(0);

impl std::fmt::Display for Done {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        // Starts new line and adds indentations.

        write!(formatter, "{}", "\n");

        let mut i = 0;

        while i < INDENTATION_COUNT.load(Ordering::Relaxed) {

            write!(formatter, "{}", "     ");

            i += 1;

        }
       
        write!(formatter, "|- ");
        
        // Writes the name of the node or "_" if empty.
        write!(formatter, "'{}' ", self.name());

        if self.is_fail() { 

            write!(formatter, "=> Failed!");

        }
        
        if self.matched_string().is_some() {
            
            write!(formatter, "=> \"{}\"", self.matched_string().unwrap());

        }

        // If this node has children then write those too.
        if self.is_nonterminal() {
                    
            // Increments `INDENTATION_COUNT` and stores the original value.
            let original_indentation_count = INDENTATION_COUNT.fetch_add(1, Ordering::Relaxed);
            
            for child in self.children().unwrap() {
                
                write!(formatter, "{}", *child);


            }

            INDENTATION_COUNT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(original_indentation_count) );

            
        }

        Ok(())

    }

}

