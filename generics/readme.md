## Generics

- Generics are <b>placeholders</b> for <b>concrete types</b>

- Generics enable writing more <b>reusable</b> and <b>flexible</b> code

- Generics allow you to avoid having <b>duplicate</b> code for different types

- Generics are a <b>zero cost abstraction</b>, the Rust compiler will, at
  compile time, fill out generics with concrete types

## Const Generics

- Const Generics are a type parameter that represents an at compile-time
  <b>constant value</b>.

- Const Generics allow you to write generic code that operates on values that
  are known at compile time

- Used for array sizes, bit widths and other constants
