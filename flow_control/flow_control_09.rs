// Loop is usually used together with `break` or `continue`.

// Fill in the blanks
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    // NOTE: from what I can tell, this is just a `while true` statement
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}
