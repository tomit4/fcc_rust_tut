// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */

// fn invalid_output<'a>() -> String {
// String::from("foo")
// }

// fn invalid_output() -> &'static str {
// "foo"
// }

fn invalid_output<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    // let x: String = invalid_output();

    // let x: &str = invalid_output();

    let s: String = String::from("foo");
    let x: &str = invalid_output(&s);

    println!("Success! x is {}", x);
}
