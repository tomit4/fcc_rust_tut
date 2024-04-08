// The impl Trait syntax works for straightforward cases
// but is actually syntax sugar for a longer form, which
// is called a trait bound.
// When working with generics, the type parameters often
// must use traits as bounds to stipulate what functionality
// a type implements.

use std::ops::Add;

fn main() {
    assert_eq!(sum(1, 2), 3);

    println!("{}", sum(5.0, 5.0));
}

// Implement `fn sum` with trait bound in two ways.
fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
