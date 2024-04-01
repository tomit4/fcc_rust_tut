## Ownership

- Rust's ownership system is unique and sets it apart from other programming
  languages
- Set of rules that govern memory management
- Rules are enforced at compile time
- If any of the rules are violated, the program won't compile

### Three Rules Of Ownership

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

**Owner:**

The owner of a value is the variable or data structure that holds it and is
responsible for allocating and freeing the memory used to store that data
