// Use `break` to break the loop.

// Fill in the blank
fn main() {
    let mut n: i32 = 0;
    for _ in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
