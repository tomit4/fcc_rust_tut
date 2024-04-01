fn main() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y` because no semicolon follows the statement
        // (i.e. implicit return)
        x_cube + x_squared + x
    };

    let z: () = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        // because this ends in a semicolon, no return value is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
