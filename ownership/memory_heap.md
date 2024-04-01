## Heap Memory

- Data of no known, fixed size belongs on the heap
- Allocating data on the heap will return a pointer (an address to location
  where data has been allocated)
- Allocating on the heap is slower than pushing to stack
- Accessing data on the heap is also slower, as it has to be accessed using a
  pointer which points to an address

### The String Type

An example of a Heap Allocated Type is The String Type

- All types covered so far were fixed size
- String is mutable
- String size can change at runtime
- String stored on the stack with a pointer to the heap
- Value of String is stored on the heap

Let's take another more in depth illustration as an example:

```rust
let s1 = String::from("hello");
```

```
s1

| name     | value |            | index | value |
| -------- | ----- |            | ----- | ----- |
| ptr      |       | =========> | 0     | h     |
| len      | 5     |            | 1     | e     |
| capacity | 5     |            | 2     | l     |
                                | 3     | l     |
                                | 4     | o     |
```

<b>Size</b> of `s1` will be <b>24 bytes</b>, 3 \* 8 bytes (usize)

- <b>ptr:</b> Pointer to data stored on the heap
- <b>len:</b> Data size in bytes
- <b>capacity:</b> Total amount of memory recieived from the allocator

### Copy vs. Move

- Scalar values with fixed sizes (all types we covered at the beginning) will
  automatically get copied in the stack, copying here is cheap
- Dynamically sized data won't get copied, but moved, copying would be too
  expensive

**Example:**

```rust
/* Here the integer value of variable `x` will get copied into `y` and
 * both variables are usable, because `i32` value has been copied, -> i32 is
 * fixed size! (i.e. though they hold the same value, `x` and `y` are pointing
 * to different locations in memory)
*/
let x = 5;
let y = x;
```

```rust
/* As `s1` is just a pointer to data on the heap,
 * Just the pointer will get copied into s2,
 * NOT the whole data on the heap!
 * (i.e. both `s1` and `s2` point to the SAME address in memory!)
*/
let s1 = String::from("hello");
let s2 = s1;
```

Note that this second example illustrates a <b>violation</b> of ownership.
Specifically this violates the <b>second rule</b> of Ownership in Rust, which
states that there can only be <b>ONE</b> owner at a time.

What the Rust Compiler will do then is to "drop" `s1` in this case, as the
declaration of `s1` will essentially "pass" ownership of the data(the string,
"hello") to `s2`, and `s1` will immediately be freed from the heap memory.

So, to reiterate, the first variable <b>s1</b> will be <b>dropped</b> and cannot
be used after assigning it to s2, to avoid dangling pointers.

### Deep Copy

```rust
// This will work, but we have to explicitly call clone() on our first string so
// that now `s2` points to a different location in heap memory
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

```
s1

| name     | value |            | index | value |
| -------- | ----- |            | ----- | ----- |
| ptr      |       | =========> | 0     | h     |
| len      | 5     |            | 1     | e     |
| capacity | 5     |            | 2     | l     |
                                | 3     | l     |
                                | 4     | o     |

s2

| name     | value |            | index | value |
| -------- | ----- |            | ----- | ----- |
| ptr      |       | =========> | 0     | h     |
| len      | 5     |            | 1     | e     |
| capacity | 5     |            | 2     | l     |
                                | 3     | l     |
                                | 4     | o     |
```
