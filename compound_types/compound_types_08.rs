// We can use `String::from` or `to_string` to convert a `&str` to `String`

// Use two approaches to fix the error and without adding a new line
fn main() {
    let s: String = "hello, world".to_string();
    let _s1: &str = s.as_str();
    // let s: String = String::from("hello, world");
    // let _s1: &str = &s;

    println!("Success!");
}
