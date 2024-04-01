// Don't use clone, use copy instead
fn main() {
    // instead of using "hello".to_string() method,
    // we pass a string literal instead, (i.e. "hello")
    // this allows us to not use x.clone() when making a copy,
    // as a string literal has a fixed size, thusly on the stack,
    // and not on the heap, so it doesn't need a call to .copy(),
    // this is done implicitly
    // NOTE also that we are passing a reference type, and not the the String type in our return
    // tuple type declarations
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
