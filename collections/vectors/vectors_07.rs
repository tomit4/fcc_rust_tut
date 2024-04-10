/*
* The elements in a vector must be the same type, for example , the code below will cause an error:
* ```
* fn main() {
*    let v = vec![1, 2.0, 3];
* }
* ```
* But we can use enums or trait objects to store distinct types.
*/

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // FILL in the blank
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}
