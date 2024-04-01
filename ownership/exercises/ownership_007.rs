fn main() {
    // Box syntax allows us to allocate memory directly on the heap
    // returning a pointer to where the data has been allocated
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1); // Implement this line, don't change other lines!

    // we cannot simply say `y = 4;` as y is defined as being an ADDRESS for an i32 type, not a
    // i32 type itself, we have to assign it directly to the POINTER to `y`
    // thusly we DEREFERENCE `y` using the `*` operator
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
