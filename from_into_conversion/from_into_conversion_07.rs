// We can use `parse` method to convert a
// `String` into a `i32` number, this is
// because `FromStr` is implemented for `i32`
// type in standard library: `impl FromStr for i32`

// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse::<i32>().unwrap();
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum: i32 = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}
