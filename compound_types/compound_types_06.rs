// You can only concat a `String` with `&str`, and `String`'s ownership can be moved to another
// variable
// Fix errors without removing any line
fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    // let s3: String = s1 + s2.as_str(); // &String -> &str
    let s3: String = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}
