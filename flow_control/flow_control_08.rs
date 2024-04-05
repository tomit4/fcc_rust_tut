// `continue` will skip over the remaining code in current iteration and go to the next iteration.

// Fill in the blanks
fn main() {
    let mut n: i32 = 0;
    for _ in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }
        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
