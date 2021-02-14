

use std::rc::Rc;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Concrete syntax tree formed from a parsing function.
#[derive(Debug)]
pub enum Done {

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

    /// An empty node - holds nothing.
    Empty()

}


impl Done {

    /// Returns this name.
    pub fn name (&self) -> Option<&'static str> {

        match self {

            Done::Terminal { name,.. } => Some(name),

            Done::Nonterminal { name, ..} => Some(name),

            Done::Empty() => None

        }

    }
    
    /// Returns children of this ``Done`` if it has any.
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

    /// Returns whether this is a terminal node.
    pub fn is_terminal(&self) -> bool {

        if let Self::Terminal {..} = self {

            return true;

        }

        false

    }
    
    /// Returns whether this is a nonterminal node.
    pub fn is_nonterminal(&self) -> bool {

        if let Self::Nonterminal {..} = self {

            return true;

        }

        false

    }

    /// Returns whether this is an empty node.
    pub fn is_empty(&self) -> bool {

        if let Self::Empty() = self {

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

// These globals just help with with `Display`
static INDENTATION_COUNT: AtomicUsize = AtomicUsize::new(0);
static CURRENT_CHILD_INDEX: AtomicUsize = AtomicUsize::new(0);

impl std::fmt::Display for Done {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {

        // Starts new line and adds indentations.

        write!(formatter, "{}", "\n");

        let mut i = 0;

        while i < INDENTATION_COUNT.load(Ordering::Relaxed) {

            write!(formatter, "{}", "     ");

            i += 1;

        }
       
        write!(formatter, "{}", "|- ");
        
        // Writes the name of the node or "_" if empty.
        write!(formatter, "{} ", self.name().unwrap_or("_"));

        if self.matched_string().is_some() {

            write!(formatter, "\"{}\"", self.matched_string().unwrap());

        }

        // If this node has children then write those too.
        if self.is_nonterminal() {
                    
            // Increments `INDENTATION_COUNT` and stores the original value.
            let original_indentation_count = INDENTATION_COUNT.fetch_add(1, Ordering::Relaxed);
            
            let original_child_index = INDENTATION_COUNT.fetch_add(0, Ordering::Relaxed);

            CURRENT_CHILD_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(0) );

            for child in self.children().unwrap() {
                
                write!(formatter, "{}", *child);

                CURRENT_CHILD_INDEX.fetch_add(1, Ordering::Relaxed);

            }

            INDENTATION_COUNT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(original_indentation_count) );

            CURRENT_CHILD_INDEX.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(original_child_index) );
            
        }

        Ok(())

    }

}


impl Clone for Done {

    fn clone(&self) -> Self {

        // If `self.is_nonterminal()` then descend this tree while cloning non-empty nodes.
        if let Done::Nonterminal { name, children} = self {

            let mut new_children: Vec<Rc<Done>> = vec![];

            // Filters empty children out of cloned results.
            let children_filtered = children.iter().filter(
            
                |child| 
                    child.is_empty()
            
            );
                
            for new_child in children_filtered {

                new_children.push(new_child.clone());

            }

            return Done::Nonterminal {

                name: name.clone(),

                children: new_children

            }

        }

        // If `self.is_terminal()` then cloning is trivial.
        else if let Done::Terminal { name, matched_string} = self {

            return Done::Terminal {

                name: name.clone(), 

                matched_string: matched_string.clone()

            }

        }

        // This is unreachable as far as I can figure.
        else { 

            panic!("Tried to call clone() on Done::Empty().")

        }

    }

}