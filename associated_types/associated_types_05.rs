// You can only make object-safe traits into trait objects.
// A trait is object safe if all the methods defined in the
// trait have the following properties:
// - The return type isn’t Self.
// - There are no generic type parameters.

// Use at least two approaches to make it work.
// DON'T add/remove any code line.

/* static dispatch approach
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    }
}

fn my_function<T: MyTrait>(x: T) -> T {
    x.f()
}

fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("Success!");
}
*/

/* dynamic dispatch approach */
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(42)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
