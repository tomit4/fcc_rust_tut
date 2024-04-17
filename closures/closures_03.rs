/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
    let movable: Box<i32> = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        // take(movable);
        take(&movable);
    };

    consume();
    // the other way is to remove the second call to consume(), as this won't work if the first
    // consume() function takes ownership of movable
    consume();
}

// fn take<T>(_v: T) {}
// One way to solve is instead of having `take()`
// take ownership of movable, we can have it take a  reference
fn take<T>(_v: &T) {}
