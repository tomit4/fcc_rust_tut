// Long tuples cannot be printed
// Fix the error
fn main() {
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); // can be used, just not
    // printed
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); // works
    println!("too long tuple: {:?}", too_long_tuple);
}
