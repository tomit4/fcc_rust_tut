// Rust provides no implicit type conversion(coercion)
// between primitive types. But explicit type conversions
// can be performed using the as keyword.

// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal: f32 = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char; // a
    let c2: char = integer as char;

    assert_eq!(integer + 1, 'b' as u8); // 98

    println!("Success!");
}
