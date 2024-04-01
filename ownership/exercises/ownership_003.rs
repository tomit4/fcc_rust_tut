fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    // Convert String to Vec
    // s.as_bytes found on official docs
    // (type `rustup docs` for downloaded local docs,
    // or visit https://doc.rust-lang.org/stable/)
    let _s = s.as_bytes();
    s
}
