// Opposite to the seldom using of `str`, `&str` and `String` are used everywhere!
// `&str` can be converted to `String` in two ways
// Fix error with at least two solutions
fn main() {
    // let s: String = String::from("hello, world");
    let s: String = "hello, world".to_string(); // &str -> String
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
