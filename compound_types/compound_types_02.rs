// We can only use `str` by boxed it, `&` can be used to convert `Box<str>` to `&str`

// Fix the error with at least two solutions
fn main() {
    // let s: Box<str> = "hello, world".into();
    // greetings(&s)
    let s: &str = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s)
}
