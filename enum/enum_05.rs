// Since there is no `null` in Rust, we have to use enum `Option<T>`
// to deal with the cases when the value is absent.

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five); // Some(6)
    let _none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        // Some(n) "unwraps" the return value of six
        // (which is 6) and encapsulates in `n`
        println!("{}", n); // 6

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
