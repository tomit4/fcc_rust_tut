## Destructuring

We can use a pattern with let to destructure a tuple to separate variables.

> Tips: You can use Shadowing or Mutability

```rust
// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
```
