// Result<T> is an enum to describe possible errors. It has two variants:

// Ok(T): A value T was found
// Err(e): An error was found with a value e

// In short words, the expected outcome is Ok, while the unexpected outcome is Err.

// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
    let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();

    Ok(n1.unwrap() * n2.unwrap()) // Ok(10) -> 10 * Ok(2) -> 2 = 20
}

fn main() {
    let result: Result<i32, ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result: Result<i32, ParseIntError> = multiply("4", "2");
    assert_eq!(result.unwrap(), 8); // Ok(8) -> 8

    println!("Success!");
}
