// make it work
use std::mem::size_of_val;

fn main() {
    let cl: char = 'a';
    assert_eq!(size_of_val(&cl), 4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
