## Array

- An Array is a <b>Fixed-size</b> collection of elements of the <b>same data
  type</b> stored as a contiguous block in <b>stack memory</b>
- The Signature of an Array is [T, Length] which indicates that the <b>length is
  fixed</b> at compile time
- Arrays can neither grow nor shrink, they must <b>retain their size</b>

**Example:**

The type of array is `[T; Length]`, as you can see, an array's length is part of
their signature. So their length must be knowna t compile time.

For example you <b>can't</b> initialize an array like below:

```rust
fn init_arr(n: i32) {
    let arr = [1; n];
}
```

This will cause an error, because the compiler has no idea of the exact size of
the array at compile time.
