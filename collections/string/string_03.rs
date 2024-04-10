// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    // let s1: &str = &s;
    // OR:
    let s1: &str = s.as_str();

    let s2: &str = &s[..5];
    assert_eq!(s2, "hello");

    let mut s3: String = s; // borrows s, as long as s is no longer used after this, you can use s
                            // in this fashion
    s3.push('!');
    assert_eq!(s3, "hello, world!");

    println!("Success!");
}
