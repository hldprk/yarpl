
  
`yarpl` is Yet Another Parsing Library which uses declarative macros.

Each macro uses a modified function declaration syntax and will expand to a function with the type `fn (&str, usize) -> Result<yarpl::core::Progress, yarpl::core::Done>`.

# Usage

`yarpl` macros will take an identifier, then a block containing that parser's one or more children.

For example to parse a simple terminal symbol you can use the `just!` macro:
    
```rust
use yarpl::just;

just!(pub fn plus { "+" } ); 
```

Then just call the function with a `&str` and an index as arguments.

```rust
assert!(plus("+", 0).is_ok());
```

Calling other parsers within a macro may be done with similarly Rust-like syntax:

```rust
use yarpl::and;

and!(pub fn plus_plus_plus {

	plus();
	plus();
	plus();

});

assert!(plus_plus_plus("+++", 0).is_ok());
assert!(plus_plus_plus("asdf", 0).is_err());
```