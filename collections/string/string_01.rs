// String type is defined in std and stored as a vector of bytes (Vec),
// but guaranteed to always be a valid UTF-8 sequence. String is heap
// allocated, growable and not null terminated.

// Fill the blank
fn main() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownerhip of \"{}\" is moved here!", s);
}
