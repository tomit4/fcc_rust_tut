// Convert any type to String

// To convert any type to `String`, you can simply
// use the `ToString` trait for that type. Rather
// than doing that directly, you should implement
// the `fmt::Display` trait which will automatically
// provides `ToString` and also allows you to print
// the type with `println!`.
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y) // write! writes to a buffer
    }
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    // because we defined fmt above, the format! macro
    // recognizes the buffer written and then saved
    // to the variable, origin
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("{}", origin);
    // The point is (0, 0)

    println!("Success!");
}
