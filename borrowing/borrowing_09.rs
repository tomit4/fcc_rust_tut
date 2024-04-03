// Ok: Borrow a mutable object as immutable
// This code has no errors!
fn main() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(_s: &String) {}
