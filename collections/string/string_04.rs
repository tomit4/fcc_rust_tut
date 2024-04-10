// Question: how many heap allocations are happening here?
// Your answer: 2
fn main() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!"); // allocating a string on the heap: 1

    // Create a slice point to String `s`
    let slice: &str = &s; // just a reference to `s`, no allocation happens here

    // Create a String type based on the recently created slice
    let s: String = slice.to_string(); // conversion to a string from the slice, heap allocation
                                       // here 2

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
