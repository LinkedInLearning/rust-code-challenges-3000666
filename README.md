# Rust Code Challenges: Challenge 10

Your challenge is to implement two functions that
handle time. They need to be able to take text
representing dates and calculate the number of weeks
between them.

- `parse_date()` takes a string as input and returns
  a `time::Date`
- `weeks_between()` takes two dates as an argument and
  returns the number of whole weeks

This challenge uses the 3rd party `time` crate, rather
than the rely purely on the standard library. The 3rd
party crate provides some richness that the standard
library's `std::time` module does not.

## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
```
