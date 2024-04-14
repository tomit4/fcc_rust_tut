fn main() {
    let s1: &str = "hello";
    /* Fill in the blank */
    let s: String = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
    println!("Success!");
}
