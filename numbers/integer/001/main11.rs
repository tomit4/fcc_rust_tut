// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    // AND(&) is basically like logical &&, only on bits alone,
    // 0 & 0 === false && false (aka "0")
    // 1 & 1 === true && true (aka  "1")
    // 0 & 1 === false && true (aka "0")
    // 1 & 0 === true && false (aka "0")
    // thusly 0011 & 0101 is like "false && false?" "false && true?" "true && false?" "true && true?"
    // resulting in "false" "false" " false" "true", and then converted back to bits: 0001

    // Similarly, OR(|) is like logical ||
    // 0 || 0 === false || false (aka "0")
    // 1 || 1 === true || true (aka "1")
    // 0 || 1 === false || true (aka "1")
    // 1 || 0 === true || false (aka "1")
    // thusly 0011 || 0101 is like "false || false?" "false || true?" "true || false?" "true || true?"
    // resulting in "false" "true" "true" "true", and then converted back to bits: 0111

    // Bitwise XOR(^) is similar to bitwise OR(|) but, returns true ONLY if the inputs are different
    // 0 ^ 0 === false (aka "0")
    // 1 ^ 1 === false (aka "0")
    // 0 ^ 1 === true (aka "1")
    // 1 ^ 0 === true (aka "1")
    // thusly 0011 ^ 0101 is like "0 !== 0?" "0 !== 1?" "1 !== 0?" "1 !== 1?"
    // resulting in "false" "true" "true" "false" or 0110

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0011 AND 0101 is 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0011 OR 0101 is 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0011 XOR 0101 is 0110

    // Bit shifting... look into it later :P
    println!("1 << 5 is {}", 1u32 << 5); // 1 << 5 is 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 >> 2 is 0x20
}
