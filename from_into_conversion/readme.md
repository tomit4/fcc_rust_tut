## From/Into Conversion

- `From` and `Into` traits are used for <b>type conversions</b> between
  different types <b>without requiring explicit casts</b>

- From/Into Conversions are part of the standard library

- Form/Into Conversions can be implemented for <b>custom types</b>

- <b>Implementing</b> `From` for a type will give us an `Into` implementation
  for the given type <b>for free</b>

### From/Into Example

Here we have created a struct `Number` with one field value. We want to be able
to convert an `i32` value directly to a `Number` type value in the `value` field.

```rust
#[derive(Debug)]
struct number {
   value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        Number {
            value: n,
        }
    }
}
```

We do this by implementing the `From` trait for our custom type `Number` and
provide the customized `from` method.

```rust
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}
```

### From/Into

The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types.

The From and Into traits are inherently linked, and this is actually part of its implementation. It means if we write something like this: `impl From<T> for U`, then we can use `let u: U = U::from(T)` or `let u:U = T.into()`.

The `Into` trait is simply the reciprocal of the `From` trait. That is, if you have implemented the `From` trait for your type, then the `Into` trait will be automatically implemented for the same type.

Using the `Into` trait will typically require the type annotations as the compiler is unable to determine this most of the time.

For example, we can easily convert &str into String :

```rust
fn main() {
    let my_str = "hello";

    // three conversions below all depends on the fact: String implements From<&str>:
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // Explicit type annotation is required here
    let string3: String = my_str.into();
}
```

Because the standard library has already implemented this for us : `impl From<&'\_ str>` for `String`.

Some implementations of `From` trait can be found [here](https://doc.rust-lang.org/stable/std/convert/trait.From.html#implementors).
