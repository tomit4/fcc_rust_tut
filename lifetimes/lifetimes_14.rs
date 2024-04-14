/* Make it work */
use std::fmt::Debug;

// Examples of Declaring Generics that MUST
// have references that have a 'static lifetime
fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

// NOTE: this version takes a REFERENCE as an argument, and thusly
// works differently when invoked in the main() function below than
// the others
fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    // let i: i32 = 5;
    // simple fix to our lifetime errors in print_it() and print_it1()
    // const i: i32 = 5;
    // also we can use static, but note the differences between static
    // and const noted in ./lifetimes_13.rs
    static i: i32 = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
