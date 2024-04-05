// For some cases, when matching enums, `match` is too heavy. We can use `if let` instead.

fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
    /*
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
    */
}
