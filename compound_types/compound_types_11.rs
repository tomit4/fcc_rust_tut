// You can't use index to access a char in a string, but you can use slice `&s1[start..end]`.

fn main() {
    let s1: String = String::from("hi,QA");
    let h: &str = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF-8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..4]; // Modify this line to fix the error, tips: `Q` takes 3 bytes in UTF-8 format

    // let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `Q` takes 3 bytes in UTF-8 format
    // (NOTE: in example they use a chinese character, unicode emoji/other languages take up more bytes, thusly the slice range is offset by this amount, and hence 6, not 4)
    assert_eq!(h1, "Q");

    println!("Success!");
}
