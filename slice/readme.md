## Slice

- A Slice is a <b>Reference</b> to a contiguous sequence of elements in a
  <b>collection</b>
- Slices provide a way to <b>borrow part</b> of a collection without taking
  <b>ownership</b> of the entire collection
- Slices can be created from arrays, vectors, Strings, and other collections
  implementing the `Deref` trait

### Example Slice

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

`slice` has the type `&[i32]` in this example.

Array slices work like string slices do, by storing a <b>reference to the first
element and a length.</b>

Slices are similar to arrays, but their length is not known at compile time, so
you can't use slice directly.
