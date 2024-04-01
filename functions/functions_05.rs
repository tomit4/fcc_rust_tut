fn main() {
    // FILL in the blank
    let b: bool = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be sued in match expression to replace a val
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
