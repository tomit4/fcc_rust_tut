// Methods are similar to functions: Declare with fn,
// have parameters and a return value. Unlike functions,
// methods are defined within the context of a struct
// (or an enum or a trait object), and their first parameter
// is always self, which represents the instance of the struct
// the method is being called on.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}
