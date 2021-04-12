`yarpl`, or **Yet Another Rust Parsing Library**, revolves around the [`Consumer`](Consumer) struct and the [`Feed`](Feed) trait.

# Consumer

A `Consumer` hold a `Vec<String>` of parsed tokens as well as the remaining unparsed `String`.


# Feed 

The `Feed` trait defines how a trait is parsed using a `Consumer` using `consume`.
