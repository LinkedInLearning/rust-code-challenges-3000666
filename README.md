# Rust Code Challenges: Challenge 13

Your challenge is to create a representation for CSS and HTML 
color codes, e.g. #RRGGBB, and implement `std::fmt::FromStr` for that
type so that it can be parsed.


## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
running 6 tests
test no_leading_hash - should panic ... ok
test invalid_literals - should panic ... ok
test not_a_hex_code - should panic ... ok
test out_of_bounds - should panic ... ok
test too_short - should panic ... ok
test every_color ... ok
...
```
