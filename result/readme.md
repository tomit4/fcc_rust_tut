## Result

- `Result` is an <b>enum</b> type that represents the <b>outcome</b> of an
  operation that <b>could potentially fail</b>

- <b>Two</b> possible variants:

  - `Ok(T)`: A value <b>T was found</b>
  - `Err(e)`: An <b>error was found</b> with a value `e`

- The <b>expected</b> outcome is <b>Ok</b>, the <b>unexpected</b> outcome is
  <b>Err</b>

- Since `Result` is an `enum`, the possible variants can be <b>matched</b> using
  a <b>match pattern</b>

### Result Example

```rust
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("Division by zero");
    }
    Ok(x / y)
}
```

The function `divide()` returns a `Result` that holds a <b>f32 float</b> in the
case of <b>success</b> and a <b>string literal</b> in the case of an
<b>error</b>. If the second argument provided is 0.0, we return an `Error`,
because <b>dividing by 0 is not allowed</b>. Otherwise, we <b>divide the two
arguments</b> and return the result <b>wrapped in Ok()</b>.

```rust
fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}
```

When calling this function, we receive a value of type `Result`. We can then
<b>match</b> that result and specify what will be the <b>output</b> in case of
<b>success</b> or in case of an <b>error</b>.

### unwrap

- The `unwrap()` method takes as input a value of <b>type Result</b> and takes
  out the value which is wrapped inside `Ok(T)` in case of success or panics in
  case of an error

### ?

- The `?` operator is a <b>shorthand</b> way to propogate errors or unwrap `Ok()` results

- Basically the same as `unwrap()` but instead of panic <b>returns an error</b>

- <b>Replaces</b> an <b>entire match statement</b>

- Can be used in the <b>main() function</b>

### Example `?` operator

```rust
fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

Can be represented as shorthand using the `?` operator like so:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(())
}
```

### Type Alias

- A Type Alias is a way of giving a new name to an existing type

```rust
type U64 = u64;

fn main() {
    let number: U64 = 42;
}
```

NOTE: Don't confuse type aliases with associated types in traits!!

### Using Result in `fn main`

Typically the main function will look like this:

```rust
fn main() {
    println!("Hello World!");
}
```

However `main` is also able to have a return type of `Result`. If an error occurs within the `main` function it will return an error code and print a debug representation of the error( Debug trait ).

The following example shows such a scenario:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    let number: i32 = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    // Another way we can annotate this is with the shorthand, `?` operator:
    // let number: i32 = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(()) // 10
}
```
