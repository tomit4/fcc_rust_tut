// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}
fn main() {
    let age: u8 = 30;
    let _p: Person = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}
