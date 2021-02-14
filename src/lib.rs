
#![allow(unused_must_use)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(extended_key_value_attributes)]


#![doc = include_str!("../README.md")]


mod done;
mod progress;
mod parse_error;
mod parse_result;
mod parser;

pub use done::Done;
pub use progress::Progress;
pub use parse_error::ParseError;
pub use parse_result::ParseResult;
pub use parser::Parser;

#[macro_use]
pub mod just;

#[macro_use]
pub mod and;

#[macro_use]
pub mod not;

#[macro_use]
pub mod same_as;

#[macro_use]
pub mod or;

#[macro_use]
pub mod repeat;

#[macro_use]
pub mod maybe;

#[cfg(test)]
mod tests {

    just!(first {"first"; });

    #[test]
    pub fn test_just () -> () {

        assert!(first("first".to_owned(), 0).is_ok());
        assert!(first("asdfasdf".to_owned(), 0).is_err());

        println!("\n{}", first("first".to_owned(), 0).unwrap().done);

    }

    just!(second { "second"; });

    and!(first_and_second { first; second; });

    #[test]
    pub fn test_and () -> () {
        
        assert!(first_and_second("firstsecond".to_owned(), 0).is_ok());
        assert!(first_and_second("firstsecond".to_owned(), 1).is_err());
        println!("\n{}\n", first_and_second("firstsecond".to_owned(), 0).unwrap());

    }

    or!(first_or_second { first; first; first; second; });
    
    #[test]
    pub fn test_or () -> () {

        assert!(first_or_second("first".to_owned(), 0).is_ok());
        assert!(first_or_second("second".to_owned(), 0).is_ok());
        assert!(first_or_second("asdfafsd".to_owned(), 0).is_err());

    }

    not!(not_first { first; });
    
    #[test]
    pub fn test_not () -> () {

        assert!(not_first("asdfsadf".to_owned(), 0).is_ok());
        assert!(not_first("first".to_owned(), 0).is_err());

    }

    repeat!(repeat_first_none_five { first; { , 5 } });
    repeat!(repeat_first_five_none { first; { 5 , } });
    repeat!(repeat_first_five_exactly { first; { 5 } });
    
    #[test]
    pub fn test_repeat () -> () {

        assert!(repeat_first_none_five("firstfirstfirst".to_owned(), 0).is_ok());
        assert!(repeat_first_five_none("firstfirstfirstfirstfirst".to_owned(), 0).is_ok());
        assert!(repeat_first_five_exactly("firstfirstfirstfirstfirst".to_owned(), 0).is_ok());
        assert!(repeat_first_five_exactly("firstfirstfirstfirst".to_owned(), 0).is_err());
        assert!(repeat_first_five_exactly("firstfirstfirstfirstfirstfirst".to_owned(), 0).is_err());

    }

    same_as!( pub same_as_first_or_second { first; second; });

    #[test]
    pub fn test_same_as () -> () {
        
        let success
            = same_as_first_or_second(String::from("first"), 0);
        
        let failure
            = same_as_first_or_second(String::from("third"), 0);
        
        println!("{}\n", success.unwrap());
        println!("{}\n", failure.unwrap_err());
        
    }

    maybe!(pub maybe_first { first; });

    #[test]
    pub fn test_maybe () -> () {

        assert!(maybe_first("first".to_owned(), 0).is_ok());
        assert!(maybe_first("asdffirst".to_owned(), 0).is_ok());
        assert!(maybe_first("".to_owned(), 0).is_ok());

    }

}