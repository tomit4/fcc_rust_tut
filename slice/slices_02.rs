/*
 * A slice reference is a two-word object, for simplicity reasons, from now on we will use slice
 * instead of `slice reference`. The first word is a pointer to the data, and the second word is
 * the length of the slice. The word size is the same as usize, determined by the processor
 * architecture, e.g. 64 bits on an x86-64. Slices can be used to borrow a section of an array, and
 * have the type signature `&[T]`.
 */

fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice: &[char] = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
