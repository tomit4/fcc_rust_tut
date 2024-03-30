// Fix errors and panics to make it work
fn main() {
    // note that these both will work, but v2 is defined in a safer fashion as
    // it uses a method(check_add) to ensure this doesn't overflow
    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}
