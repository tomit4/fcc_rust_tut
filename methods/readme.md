## Methods

- A Method is a Function that is <b>associated</b> with a <b>particular type</b>
  or <b>struct</b>

- A Method takes <b>parameters</b> and <b>returns a value<b>. but is defined as
  a <b>member</b> of a struct or enum

- Methods are called using <b>dot notation</b> (like accessing members of a
  struct)

- Methods are implemented through an `impl` block

### Example Method

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Inside the `imple` block for the type `Rectangle` we can define the method
`area()` that doesn't take any arguments and <b>returns the product</b> of width
and height of an instance as a `u32` integer.

```rust
let rectl = Rectangle {
    width: 30,
    height: 50,
};

println!(
    "The area of the rectangle is {} square pixels.",
    rectl.area()
)
```

Here, we create an instance of `Rectangle` with values for width and height.
Then we can call the method using dot notation on the instance we've created.

<b>self</b> refers to the instance the method is called upon, in this case
`rect1`.

## Associated Functions

- An Associated Function is a Function that is associated with a struct or an
  enum, but <b>doesn't take an instance</b> as its first parameter

- Associated Functions are called using the name of the type, not an instance of
  it

- Associated Functions are often used as <b>constructors</b> for a struct or
  enum

### Associated Function Example

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
```

`new()` is an <b>associated function</b>, because it doesn't have "self" as its
first parameter.

So, to call the `new()` function, we don't need an instance of the struct.

```rust
let rec1: Rectangle = Rectangle::new(5, 10);

println!("Rectangle: {:?}", rec1);
```

We can then call the associated function by using the name of the struct and the
method name separated by "::".
