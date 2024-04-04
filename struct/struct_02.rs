/*
 * Unit struct don't have any fields. It can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
 */

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {}
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(_u: Unit) {}
