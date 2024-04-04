// String slices
fn main() {
    let s: String = String::from("hello");

    let slice1: &str = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2: &str = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
