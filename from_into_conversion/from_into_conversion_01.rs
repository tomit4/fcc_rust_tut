fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into(); // 0
    let i2: i32 = i32::from(false); // 0
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char> for u32`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let i32: u32 = 'a'.into();

    // FIX the error in two ways
    // let s: String = String::from('a');
    let s: String = 'a'.into();

    println!("Success!");
}
