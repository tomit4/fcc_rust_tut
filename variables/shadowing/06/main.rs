// Remove a line in the code to make it compile
fn main() {
    let mut _x: i32 = 1;
    _x = 7; // x = 7

    // Shadowing and re-binding
    // let mut x = x; // 7
    _x += 3;

    let _y: i32 = 4;
    // Shadowing
    let _y: &str = "I can also be bound to text!";

    println!("Success!");
}
