## Variables

Binding and mutability

!. A variable can be used only if it has been initialized

```rust
// Fix the error below with the least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
```
