# Rust Code Challenges: Challenge 1

Your challenge is to implement a function, `median()`, that takes a vector of floating point numbers (`Vec<f32>`) and returns the median as a floating point number.

Your return value should be wrapped in an `Option` type to account for cases where the input list is empty. When that occurs, there is no meaningful median.

## Testing your code

To test your solution, use `cargo test`.

```console
$ cargo test
...
running 2 tests
test empty_input ... ok
test one_two_five ... ok
...
```