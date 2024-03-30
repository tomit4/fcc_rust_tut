## Shadowing

You can declare a new variable with the same name as a previous variable, here we can say the <b>first one is shadowed by the second one.</b>

```rust
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}
```
