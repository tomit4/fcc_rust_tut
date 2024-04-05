## Enum

- Enums are a way of defining a type with only one of a possible set of values
- You can only access one variant of an enum at a time
- Enums can hold additional information using tuples
- Enums are especially useful when used in `match` statements

**Example Enum**

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Enum for IP addresses. An IP address can <b>either</b> be of <b>V4 format</b> or
<b>V6 format</b>. Each variant in the enum holds a `String` value.

## The Option Enum

- Option is an enum that represents a value that <b>may or may not be
  present</b>
- Known in other langauges as <b>"null"</b>, referring to the <b>absence of a
  value</b>
- Used to handle cases where a function or method <b>might fail</b> to
  <b>return</b> a value

  **Example:**

```rust
// Option enum as defined by the standard library
enum Option<T> {
  None,
  Some(T), // example of a Generic (to be covered later)
}
```

```rust
fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
}

/*
 * `plus_one()` expects an argument of type
 * <b>Option</b>, so we have to wrap an
 * `i32` inside `Some()`
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    /*
     * `x` gets <b>matched</b> and if there is a
     * `Some()` value, it gets incremented
     * by one.
    */
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
