// You can't use index to access a char in a string, but you can use slice &s1[start..end].

fn main() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1: &str = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    // Iterate through all chars in s
    for (i, c) in s1.chars().enumerate() {
        if i == 6 {
            assert_eq!(c, '中')
        }
    }

    println!("Success!");
}
