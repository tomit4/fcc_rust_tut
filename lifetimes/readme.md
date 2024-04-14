## Lifetimes

- Lifetimes are another <b>kind of generic</b> that ensures that
  <b>references</b> are <b>valid</b> as long as needed

- Every <b>reference</b> has a <b>lifetime</b>, which is the <b>scope</b> for
  which that reference is valid

- Most of the time, lifetimes are <b>implicit</b> and <b>inferred</b>, meaning
  there's no need to worry about <b>annotating</b> them

- Sometimes though, lifetime <b>annotations</b> are needed, if the compiler cannot
  infer it

- Lifetime annotations are a concept which most other programming languages
  don't have

### Dangling References

The main aim of lifetimes is to <b>prevent dangling references</b> (also known
as dangling pointers).

Here in the following example code, the outer score declares variable `r` with
no initial value. The inner scope declares variable `x` with an initial value of
`5`. Then a reference of `x` is assigned to `r`.

`x` then goes out of scope and `r` that `x` refers to has gone out of scope.

```rust
fn main () {
    let r;                // ----------+-- 'a
                          //           |
    {                     //           |
        let x = 5;        // -+-- 'b   |
        r = &x;           //  |        |
    }                     // -+        |
                          //           |
    println!("r: {}", r); // ----------+
}
```

### Borrow Checker

- The Borrow Checker <b>compares scopes</b> to determine whether <b>all borrows
  are valid</b>

- The Borrow Checker is a key part of Rust's <b>ownership</b> system

- The Borrow Checker tracks the lifetimes of <b>references</b> and ensures that
  they don't violate the <b>ownership rules</b>

- The Borrow Checker's Ownership Rules ensure that a value is <b>not
  accessed</b> after it has been <b>moved</b> or <b>freed</b> from memory

- <b>Important:</b> A reference to a value <b>must never outlive the value
  itself!</b>

### Three Rules Of Lifetime Elision

- The compiler uses <b>three rules</b> to figure out lifetimes of references
  that aren't <b>explicit</b> annotation

1. The Compiler <b>assigns a lifetime</b> parameter to each parameter that's a
   reference

2. If there is <b>exactly one</b> input lifetime parameter, that lifetime is
   assigned to <b>all output</b> lifetime parameters

3. If there are multiple lifetime parameters, but one of them is <b>`&self`</b>
   or <b>`&mut`</b> self, the <b>lifetime of self is assigned to all output</b>
   lifetime parameters (i.e. the lifetime of a `struct` extends to all its
   methods)

### Example Of LifeTime Elision

**Example 01:**

```rust
fn first_word(s: &str) -> &str {
```

=>

```rust
fn first_word<'a>(s: &'a str) -> &str {
```

=>

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

The Compiler applies the <b>first rule</b>: Each parameter gets its own
lifetime.

The <b>second rule</b> applies because there is exactly <b>one input</b>
lifetime, so the lifetime of the one input parameter gets assigned to the output
lifetime.

In this case, the compiler could <b>infer</b> the lifetime and we don't have to
specify them <b>manually</b>.

**Example 02:**

```rust
fn longest(x: &str, y: &str) -> &str {
```

=>

```rust
// compiler would complain here because the return type is of an uknown lifetime
// (is it `a or `b ?)
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

First rule: Each parameter gets its own lifetime

Here the second rule doesn't apply because there is more than one input
lifetime. Also, the third rule doesn't apply because this function is not a
method.

We have to manually annotate the lifetime parameters.

**Example 03:**

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

Here the <b>third rule</b> applies, which states that if there is a reference
to self(`&self`), then <b>all references will have the same lifetime as
`&self`</b>.

### Static Lifetime

- A Static Lifetime refers to a lifetime that lasts for the <b>entire
  duration</b> of the program's <b>execution</b>

- Any reference or borrowed value with a static lifetime can be
  <b>safely</b> used throughout the program

- A Static Lifetime can be <b>coerced</b> to a <b>shorter</b> lifetime if needed

**Example Static Lifetime**

```rust
let s: &'static str = "Hello, world!";
```

String literals have a <b>static</b> lifetime because they are <b>hardcoded</b>
into the executable, meaning that they are <b>valid</b> throughout the
<b>entire duration</b> of the program's execution.
