// When casting any value to an unsigned type T,
// T::MAX + 1 is added or subtracted until the
// value fits into the new type.
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256); // u8::MAX + 1 = 255 + 1 = 256

    assert_eq!(-1_i8 as u8, 255);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, 255); // largest possible number u8 can represent (255)
    assert_eq!(-100.1_f32 as u8, 0); // smallest possible number u8 can represent (0)

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44 -> 300 - u8::MAX + 1 = 300 - 256 = 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156 -> u8:MAX + 1 = 256 - 100 = 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
