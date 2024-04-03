// Fix error
fn main() {
    let mut _s: String = String::from("hello, ");

    borrow_object(&_s);

    println!("Success!");
}

fn borrow_object(_s: &String) {}
