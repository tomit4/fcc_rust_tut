// Fill in the blank
enum Foo {
    Bar(u8),
}

fn main() {
    let a: Foo = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
