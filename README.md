`yarpl` (or **Yet Another Rust Parsing Library**) makes use of a the `Consumer` struct and the `Feed` trait to make parsing easier.

# Consumer

A `Consumer` tracks parsed tokens as well as some remaining unparsed input.


# Feed 

The `Feed` trait defines how a trait is parsed using a `Consumer` using `Consumer::consume`.
