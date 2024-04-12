// Similar to From and Into, TryFrom and TryInto
// are generic traits for converting between types.

// Unlike From/Into, TryFrom and TryInto are used for
// fallible conversions and return a Result instead of
// a plain value.

// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope (incorrect, had to uncomment to get try_into() to work)
use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method ?
    // conversion from i16 to u8 fails because 256 is out of bounds for a u8 type,
    // thusly this will return an Err and n will be returned to as 0
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}
