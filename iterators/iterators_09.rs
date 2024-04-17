/* Make it work */
use std::collections::HashMap;

fn main() {
    let names: [(&str, i32); 2] = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<&str, i32> = names.iter().cloned().collect(); // Clone the references

    println!("{:?}", folks);

    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().cloned().collect(); // Clone the integers

    assert_eq!(v2, vec![1, 2, 3]);
}
