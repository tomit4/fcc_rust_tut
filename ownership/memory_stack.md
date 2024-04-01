## Memory

- Component in a computer to store data and instructions for the processor to
  execute
- Random Access Memory (RAM) is volatile, when power turned off all contents are
  lost.
- Two types of regions in RAM used by a program at runtime: Stack memory and
  heap memory.

### Stack Memory

- Last in, first out
- All data stored on the stack must have a known, fixed size (like integers,
  floats, char, bool, etc...)
- Pushing to the stack is faster than allocating on the heap, because the
  location for new data is always at the top of the stack
- Types of unknown size will get allocated to the heap and a pointer to the
  value is pushed to the stack, because a pointer is fixed size (usize)

**Example:**

```rust
fn main() {
    let x = 42;
    let y = 10;
    let z = add_numbers(x, y);

    println!("The result is {}", z);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
```

Call Sack:

`main()` is a function and added to the <em>heap</em> memory. A pointer to
`main()` is pushed onto the <em>stack</em>. Then `x`, `y`, and `z` are added to
the <em>stack</em>. A pointer to `add_numbers()` is added to the <em>stack</em>
and the function `add_numbers()` is added to the <em>heap</em>.

The variables local to `add_numbers()`, `a`, `b`, and `c` are added to the stack.

`c` is returned from `add_numbers()`, the pointer to `add_numbers()` is freed from
<em>heap</em> memory, and `add_numbers()` is popped off the <em>call stack</em>.

The call to `println!` in `main()` executes(itself a part of this pattern...),
and the `main()` function is called off the <em>call stack</em>, and the
pointer to `main()` is freed from memory. With no other functions remaining
on the <em>call stack</em>, our program exits.

```
|               |
|               |
|_______________|
| add_numbers() |
|   a, b, c     |
|_______________|
|    main()     |
|   x, y, z     |
|_______________|
```
