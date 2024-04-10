// FIX the errors with least changes
// DON'T remove any code line
use std::collections::HashMap;
fn main() {
    let v1: i32 = 10;
    let mut m1: HashMap<i32, i32> = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2: String = "hello".to_string();
    let mut m2: HashMap<&str, i32> = HashMap::new();
    // Ownership moved here
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}
