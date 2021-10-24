# Rust Code Challenges: Challenge 5

Your challenge is to implement a trait, `MorseCode` for `String`.
The `MorseCode` trait contains a method `to_morse_code()` that 
returns a `Message`.

The `Message` type is defined within the sample code. It is a
type alias for `Vec<Letter>`, where `Letter` represents a single
character from the Morse code alphabet (A-Z, 0-9).

As a refresher on Morse code, individual characters are made up 
of short bursts of "pulses", which can either be short (`.`) or 
long (`_`).

Here are a few characters from the alphabet and their Morse code
equivalents:

<table>
  <tr><td>A</td><td><code> _. </code></td></tr>
  <tr><td>B</td><td><code> _... </code></td></tr>
  <tr><td>C</td><td><code> _._. </code></td></tr>
  <tr><td colspan=2>...</td></tr>
  <tr><td>X</td><td><code> _.._ </code></td></tr>
  <tr><td>Y</td><td><code> _.__ </code></td></tr>
  <tr><td>Z</td><td><code> __.. </code></td></tr>
</table>

To represent pulses in Rust, we'll use an enum:

```rust
enum Pulse {
    Short,
    Long,
}
```

Each character in the alphabet (A-Z, 0-9) takes up a variable number
of pulses to be represented. Letters that occur more frequently in
English, such as E and A, take fewer letters to be represented. To
accomodate this, we'll represent a `Letter` as a `Vec<Pulse`>.

```rust
type Letter = Vec<Pulse>;
```

With `Letter` defined, we're able to build messages. A `Message` is
defined as:

```rust
type Message = Vec<Letter>;
```

It is a 
type alias for the `Vec<

## Testing your code

To test your solution, use `cargo test`.

```console
$ cargo test
...
```
