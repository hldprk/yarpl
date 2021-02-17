


#[cfg(test)]
mod tests {
	
	use yarpl::just;
	use yarpl::and;
	use yarpl::or;
	use yarpl::maybe;
	use yarpl::not;
	use yarpl::repeat;
	use yarpl::same_as;
	
    just!(fn first { "first" });

    #[test]
    pub fn test_just () -> () {

        assert!(first("first", 0).is_ok());
        assert!(first("asdfasdf", 0).is_err());

        println!("\n{}", first("first", 0).unwrap().done().unwrap());

    }

    just!(fn second { "second" });

    and!(fn first_and_second { first(); second(); });

    #[test]
    pub fn test_and () -> () {
        
        assert!(first_and_second("firstsecond", 0).is_ok());
        assert!(first_and_second("firstsecond", 1).is_err());
        println!("\n{}\n", first_and_second("firstsecond", 0).unwrap());

    }

    or!(fn first_or_second { first(); first(); first(); second(); });
    
    #[test]
    pub fn test_or () -> () {

        assert!(first_or_second("first", 0).is_ok());
        assert!(first_or_second("second", 0).is_ok());
        assert!(first_or_second("asdfafsd", 0).is_err());

    }

    not!(fn not_first { first(); });
    
    #[test]
    pub fn test_not () -> () {

        assert!(not_first("asdfsadf", 0).is_ok());
        assert!(not_first("first", 0).is_err());

    }

    repeat!(fn repeat_first_five_times { first(5, 5); });
    #[test]
    pub fn test_repeat () -> () {

        assert!(repeat_first_five_times("firstfirstfirstfirstfirst", 0).is_ok());

    }

    same_as!( pub fn same_as_first_or_second { first(); second(); });

    #[test]
    pub fn test_same_as () -> () {
        
        let success
            = same_as_first_or_second("first", 0);
        
        let failure
            = same_as_first_or_second("third", 0);
        
        println!("{}\n", success.unwrap());
        println!("{}\n", failure.unwrap_err());
        
    }

    maybe!(pub maybe_first { first; });

    #[test]
    pub fn test_maybe () -> () {

        assert!(maybe_first("first", 0).is_ok());
        assert!(maybe_first("asdffirst", 0).is_ok());
        assert!(maybe_first("", 0).is_ok());

    }

}