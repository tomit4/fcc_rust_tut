trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// IMPLEMENT below with generics.
// note that static dispatches are faster because there is
// NOTE: a dynamic lookup of the types at runtime, RATHER, the lookup is done
// WITHIN THE COMPILED BINARY
fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

// Implement below with trait objects.
// NOTE: Despite the slower runtime of dynamic dispatched associated types, sometimes
// you need to dynamically allocate memory because you cannot know the amount of memory needed to
// be allocated at compile time
fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
