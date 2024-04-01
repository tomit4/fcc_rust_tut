fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    // you can think of match like a switch/case statement
    match tp {
        // if tp == 1, then this code block executes
        1 => {
            // TODO
        }
        // otherwise if defined with an underscore `_`,
        // then this is our default case
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// Implement this function in THREE ways
fn never_return_fn() -> ! {
    panic!()

    // use unimplemented macro if you have a function that is not implemented yet...
    // unimplemented!()

    // similar to unimplemneted!()
    // todo!()
}
