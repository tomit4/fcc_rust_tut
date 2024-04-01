// solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

// This is what is called a diverging function, a function that never returns to the caller, so
// they may be used in places where a value of any type is expected
// why you might need diverging functions is clarified well enough by this stack overflow post:
// https://stackoverflow.com/questions/70813053/what-is-the-point-of-diverging-functions-in-rust
fn never_return() -> ! {
    // Implement this function don't modify the fn signatures
    panic!()
}
