# Rust Code Challenges: Challenge 7

Your challenge is to implement two functions that
handle time. 

This challenge uses the 3rd party `time` crate, rather
than the standard library. The 3rd party crate provides
some richness that the standard libary's `std::time` 
module does not.

- `parse_date()` takes a string as input and returns
  a `time::Date`
- `weeks_between() takes two dates as an argument and
  returns the number of whole weeks


## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
```
