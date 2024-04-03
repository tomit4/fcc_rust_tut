## Borrowing

- Borrowing is a way of temporarily accessing data <b>without taking
  ownership</b> of it
- When borrowing, you're taking a <b>reference</b> (pointer) to the data, not
  the data itself
- Prevention of dangling pointers and data races
- Data can be borrowed <b>immutabily</b> and <b>mutably</b>
- There are certain rules when borrowing which we have to comply with, otherwise
  the program won't compile

## Rules of References

1. At any given time, you can have either <b>one mutable reference</b> or <b>any
   number</b> of <b>immutable references</b>
2. References must <b>always be valid</b>

## Example Reference

```
|      s       |       |      s1      |
| name | value |       | name | value |         | index | value |
|  ptr |       | ----->|  ptr |       | ------->|   0   |   h   |
                       | len  |   5   |         |   1   |   e   |
                       | capacity | 5 |         |   2   |   l   |
                                                |   3   |   l   |
                                                |   4   |   o   |
```

```rust
fn main() {
    // remember, s1 is not the data, "hello",
    // but rather is a POINTER to the data "hello"
    let s1 = String::from("hello");

    // we then pass the REFERENCE to s1 to the calculate_length() function
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // we then return the len() of the reference, `s`
    s.len()
}
```

## Example Mutable Reference

```rust
fn main() {
    let mut s = String::from("hello");
    // we now explicitly state that we want to allow change() to mutate the data
    // REFERENCED by `s`
    change(&mut s);
}

fn change(some_string: &mut String) {
    // notice that we don't return anything here, we simply call the push_str on
    // some_string, which changes the variable `s` directly in the main()
    // function
    some_string.push_str(", world");
}
```

## Example Borrowing Rules

**Example 01:**

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

The above code snippet exemplifies the <b>violation of the first rule</b> of
borrowing, which says that we can only have <b>ONE mutable reference</b> to the
same data at a time!

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r 1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

**Example 02:**

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

// you can't use both immutable and
// mutable references all at once
println!("{}, {}, and {}", r1, r2, r3);
```

The above code snippet exemplifies the <b>violation of the first rule</b> of
borrowing, which says that we can <b>either</b> have <b>any number</b> of
immutable references or <b>one single mutable</b> reference. The following code
demonstrates this separation of using both mutable and immutable references
properly:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

## Example Dangling Reference

```rust
fn main() {
    // rust compiler will not compile because while this does hold onto &s,
    // the memory from s is not "freed" when dangle()'s scope ends
    let reference_do_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    // because we return the REFERENCE to s, but we instantiated s above,
    // this means that `s` will be dropped once the program reaches the end of
    // this function's scop
    &s
}
```

The above code snippet will <b>violate</b> the <b>second rule</b> which states
that references must <b>ALWAYS</b> be <b>valid</b>. The below code shows the
subtle difference that will pass the rust compiler check:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
