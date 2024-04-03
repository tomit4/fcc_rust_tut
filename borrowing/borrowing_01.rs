fn main() {
    let x: i32 = 5;
    // Fill the blank
    let p: &i32 = &x;

    println!("the memory address of x is {:p}", p); // one possible output: 0x16fa3ac
}
