//  Loop is an expression, so we can use it with break to return a value

// Fill in the blank
fn main() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            // NOTE: see how we can return
            // the value after the break statement
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}
