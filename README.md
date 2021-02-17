
  
`yarpl` is Yet Another Parsing Library which uses declarative macros.

Each macro uses a modified function declaration syntax and will expand to a function with the type `fn (&str, usize) -> Result<Progress, Done>`.

# Usage

`yarpl` macros will take an identifier, then a block containing that parser's one or more children.

For example, parsing a simple terminal symbol can be done by calling the `just!` macro like so:
    
```rust
just!( pub plus { "+" } ); 
```

Then, just call the function with some source `&str` and an index.

```rust
let result = plus( "+", 0 );

assert!( result.is_ok() );
```
