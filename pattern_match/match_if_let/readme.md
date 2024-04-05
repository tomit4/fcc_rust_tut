## Pattern Match

- Pattern Matching is a powerful construct that allows us to <b>compare a value</b> against a set of
  <b>patterns</b>, then <b>execute</b> different code based on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, etc...
- When a pattern is matched, <b>all possible cases</b> must be handled, which is enforced by the compiler

**Example Match:**

```rust
/*
 * Here, we have an `enum Coin` which holds
 * <b>different denominations</b> of US coins.
*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/*
 * `value_in_cents()` takes as an
 * argument a `Coin` (which can only
 * <b>hold one field</b> of the enum) and
 * checks which field in the enum has been
 * <b>matched</b>. Then returns the right
 * amount as `u8`.
*/
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### if let

In a match statement, every case has to be handled. This sometimes leads to
annoying boilerplate code that is not necessary. Instead we can use if let to
unwrap a value of an Option type.

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (), // this is too verbose
}
```

Instead of using this verbose `match` statement, we can instead use the `if let`
statement like so:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maxiumum is configured to be {}", max);
}
```

This is useful if we are <b>sure</b> that we only have to handle one case of
pattern matching, where a single conditional statement will return something, or
it will never run and there are no extraneous conditions that could possibly
execute.
