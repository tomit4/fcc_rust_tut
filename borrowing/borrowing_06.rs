// Ref
//
// `ref` can be used to take references to a value, similar to &.

fn main() {
    let c: char = 'Q';

    let r1: &char = &c;
    // Fill the blank, don't change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r2), get_addr(r2));

    println!("Success!");
}
// Get memory address of string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
