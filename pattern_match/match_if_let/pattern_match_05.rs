enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        // if e == MyEnum::Foo { // You can't compare enum properties using equality operators
        // count += 1;
        // }
        // instead use pattern matching using matches! macro
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}
