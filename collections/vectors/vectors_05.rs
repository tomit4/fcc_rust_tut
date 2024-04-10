// A Vec can be mutable. On the other hand,
// slices are read-only objects. To get a slice, use `&`.

// In Rust, it’s more common to pass slices as arguments
// rather than vectors when you just want to provide read access.
// The same goes for `String` and `&str`.

// FIX the errors
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: &[i32] = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    // let slice2: &[i32] = &v[0..4];
    let slice2: &[i32] = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];
    // slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}
