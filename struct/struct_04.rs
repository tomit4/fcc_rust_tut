/*
 * You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.
 */

// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age: u8 = 18;
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}
