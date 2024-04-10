// A Vec can be extended with extend method

// FILL in the blank
fn main() {
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop(); // [1, 2]
    v1.push(3); // [1, 2, 3]

    let mut v2: Vec<i32> = Vec::new();
    v2.extend(&v1); // [1, 2, 3]

    assert_eq!(v1, v2);

    println!("Success!");
}
