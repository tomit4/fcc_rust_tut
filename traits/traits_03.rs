// In Rust, many of the operators can be overloaded via traits.
// That is, some operators can be used to accomplish different
// tasks based on their input arguments. This is possible because
// operators are syntactic sugar for method calls. For example,
// the + operator in a + b calls the add method (as in a.add(b)).
// This add method is part of the Add trait. Hence, the + operator
// can be used by any implementor of the Add trait.

use std::ops;

// Implement fn multiply to make the code work.
// As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}
