use std::{error::Error};


/// Debugging information traced from failed parse.
#[derive(Clone)]
pub struct ParseError {

    /// How many characters the failed parsing function had progressed.
    pub offset: usize,

    /// Stack of names of parsing functions that the error originated from.
    pub name_stack: Vec<&'static str>,
    
    /// A string describing the error - set by any parsing function that it passes through.
    pub message: &'static str

}

impl ParseError {


    /// Common implementation of `fmt` for `Debug` and `Display`.
    ///
    /// For now this will just print the name and a block of its lower nodes.
    /// (for example something like "some_parser { first_function, second_lower_function }".) I do however want to
    /// change this to a typical tree-like output, where all lower nodes are shown too.
    fn fmt_for_debug_and_display(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut output = String::from("");

        let mut stack = self.name_stack.clone();

        stack.reverse();

        let mut i = 0;

        output.push_str(self.message);

        while i < stack.len() {

            output.push_str("\n     at ");

            let name = stack[i];

            output.push_str("\'");
            output.push_str(name);
            output.push_str("\'");

            i+= 1;

        }

        output.push_str("\n");

        write!(formatter, "{}", output)

    }

}


impl std::fmt::Debug for ParseError {

    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        self.fmt_for_debug_and_display(formatter)

    }

}



impl std::fmt::Display for ParseError {

    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        self.fmt_for_debug_and_display(formatter)

    }

}


impl Error for ParseError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {

        None
    
    }
    
}