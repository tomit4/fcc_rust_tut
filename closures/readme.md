## Closures

- Closures are <b>anonymous functions</b> that are able to <b>capture</b> values
  from the scope in which they are defined

- Closures can be defined <b>inline</b> (for example, as a function parameter)

- Closures <b>don't require type annotations</b>

- Closures can take <b>ownership</b> of a value by using the <b>`move`</b>
  keyword

### Fn Traits

- A Fn Trait is a Trait that defines a <b>signature</b> for closures/functions

- Fn Traits describe <b>types</b>, number of <b>arguments</b>, and a <b>return
  type</b>

- There are three different Fn Traits:

  - <b>FnOnce</b>

    - A Closure that can be called once
    - A Closure that takes ownership of captured values

  - <b>FnMut</b>

    - A Closure that might mutate captured values
    - A Closure that can be called more than once

  - <b>Fn</b>
    - A Closure that doesn't take ownership fo captured values
    - A Closure that doesn't mutate anything
    - A Closure that might not even capture anything from its environment

### Closure example

```rust
fn main() {
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);
}
```

This closure <b>captures</b> the value of x and <b>modifies</b> it.
The Compiler will capture variables in the <b>least restrictive</b> manner
possible.

In this case, a <b>mutable reference of x<b> is taken, rather than taking
ownership because it's less restrictive.
