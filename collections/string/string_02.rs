// Opposite to the seldom using of str, &str and String are used everywhere!

// Fix error with at least two solutions
fn main() {
    let s = String::from("hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
