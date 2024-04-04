## Struct

- A Struct is a Compound Type that allows for the <b>grouping together</b> of
  values of different types into a <b>named data structure</b>
- Structs are similar to tuples, but each value in a Struct has a <b>name</b>,
  so values can be <b>accessed</b> through this name
- Structs have to be <b>instantiated</b> with data. It is helpful to think of
  Structs as a <b>template for the instances<b> you create from it

### Instantiating a Struct

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

### Accessing and Mutating

```rust
let mut user1 = User { // Note the `mut` keyword here!
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
}

// We can <b>access</b> and <b>mutate</b> single fields of structs:
user1.email = String::from("anotheremail@example.com");
```

### Functions returning Structs

```rust
// Functions can <b>instantiate</b> and <b>return</b> structs
fn build_user(email: String, username: String) -> User {
    // Note here that since the types are defined in our function signature,
    // we do not have to repeat ourselves when instantiating those fields within
    // our `User` struct
    User {
        active: bool,
        username,
        email,
        sign_in_count: u64,
    }
}
```

### Struct Update Syntax

```rust
fn main() {
    let user1 =  User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

```rust
// Changes only the email field, takes everything else from user1
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

```rust
// Does the same as the above example, but is less verbose
let user2 = User {
    email: String::from("another@example.com"),
   ..user1
};
```

### Tuple Structs

- Tuple Structs are like normal structs but using tuple-like syntax for defining
  their fields
- Basically, Tuple Structs are just named tuples
- Tuple Structus are instantiated by parentheses('()') instead of curly
  braces('{}')
- Like Structs, Tuple Structs are accessed through point ("dot") notation.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like Structs

- Unit-Like Structs are Structs without any field
- Unit-Like Structs are used when working with traits (to be covered in a future
  section!)
- Unit-Like Structs don't store any data
