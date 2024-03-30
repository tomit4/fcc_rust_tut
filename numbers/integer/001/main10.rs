// Fill the blanks
// pulling in standard library, ops
use std::ops::{Range, RangeInclusive};
fn main() {
    // asserting that 1..5 is equivalent to a tuple of (1, 2, 3, 4, 5)
    // NOTE that according to fcc tut, this first use of
    // Range is not as commonly used as RangeInclusive
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
