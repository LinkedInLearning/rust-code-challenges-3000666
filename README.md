# Rust Code Challenges: Challenge 6

Your challenge is to implement code that calculates the
value of a hand of cards for the game of blackjack.

Cards have the following values:

- Cards **2** to **9** have a value equal to their number
- Face cards (the **King**, **Queen** and **Jack** have a value of 10
- The **Ace** has a value of 11, unless the total value of
  the hand exceeds 21. If that happens, the card's value is 1.
- Suits are not significant
- There are no other cards, such as Jokers, to consider

## Testing your code

To test your solution, use `cargo test`.

```console
$ cargo test
...
```
