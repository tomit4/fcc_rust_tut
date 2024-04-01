## Scope

- Range within a program for which an item is valid
- <b>Global scope:</b>
  - Accessible throughout the entire program
- <b>Local scope:</b>

  - Accessible only within particular function or block of code
  - Not accessible outside of that function or block

  **Example:**

```rust
// s is not valid here, it's not yet declared
{
    let s = "hello"; // s is valid from this point forward
    // do stuff with s
}
// this scope is now over, and s is no longer valid
```

- When `s` comes into scope, it is valid.
- `s` remains valid until it goes out of scope.
- <b>General rule:</b> Scope ends where block of code ends (curly brackets)
