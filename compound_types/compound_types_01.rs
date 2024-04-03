// The type nof string literal "hello, world" is &str, e.g. let s: &str = "hello, world".
//
// Str and &str
// We can't use `str` type in normal ways, but we can use &str.
// Fix error without adding new line
fn main() {
    let s: &str = "hello, world";

    println!("Success!");
}
