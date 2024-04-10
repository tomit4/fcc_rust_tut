## Associated Types

- Associated Types allow you to specify a <b>type</b> that is <b>associated</b>
  with the trait

- When <b>implementing</b> the trait for a specific type, we have to specify the
  <b>concrete type</b>

- Basically, an associated type is a type <b>placeholdeer</b> that the trait
  methods can use in their <b>signature</b>

- Associated Types are similar to <b>generic types</b> but are more
  <b>flexible</b> because they allow a trait to have <b>different associated</b>
  types for <b>different implementing</b> types

  **Associated Type Example**

Here we define a trait that has an associated type and a method that returns a
value of this type.

```rust
trait MyTrait {
  type MyType;

  fn get_my_type(&self) -> Self::MyType;
}
```

When <b>implementing</b> the trait for a specific type (`MyStruct`), then we
have to give the associated type `MyType` a concrete type, in this case `i32`.

```rust
struct MyStruct {}

impl MyTrait for MyStruct {
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}
```
