fn main() {
    // this now works because the lifetime lives here now
    let static_string: &'static str = "I'm in read-only memory";
    {
        // Make a `string` literal and print it:
        // let static_string: &'static str = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, BUT THE DATA REMAINS IN THE BINARY.
    }

    println!("static_string reference remains alive: {}", static_string);
}
