// make it work with two ways
fn main() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    // other way, throws a warning though as x is never used
    let v2: () = {
        let mut x: i32 = 1;
        x += 2
    };

    assert_eq!(v2, ());
    println!("Success!");
}
