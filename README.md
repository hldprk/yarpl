`yarpl`, or **Yet Another Rust Parsing Library**, revolves around the [`Consumer`](Consumer) struct and the [`Feed`](Feed) trait.

# Consumer

A `Consumer` hold a `Vec<String>` of parsed tokens as well as the remaining unparsed `String`.

Parsing individual tokens is done by using `Consumer::shift` and `Consumer::shift_characters` methods.

# Feed 

The `Feed` trait defines how a trait is parsed using a `Consumer` using a combination of `shift`, `shift_characters`, and `consume`.
