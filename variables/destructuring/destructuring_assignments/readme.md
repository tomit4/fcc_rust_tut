## Destructuring assignments

Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

```rust
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
}
```
