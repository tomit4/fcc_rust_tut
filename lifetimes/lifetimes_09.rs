/* Remove all the lifetimes that can be elided */

// fn input<'a>(x: &'a i32) {
// println!("`annotated_input`: {}", x);
// }

// One argument, compiler can infer lifetime
fn input(x: &i32) {
    println!("`annotated_input`: {}", x);
}

// fn pass<'a>(x: &'a i32) -> &'a i32 {
// x
// }

// One argument, returns a reference, again: compiler can infer lifetime
fn pass(x: &i32) -> &i32 {
    x
}

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
// x
// }

// Left the same because we are telling the compiler that both references
// 'a and 'b must outlive the longest() function
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

// impl Owner {
// // Annotate lifetimes as in a standalone function.
// fn add_one<'a>(&'a mut self) {
// self.0 += 1;
// }
// fn print<'a>(&'a self) {
// println!("`print`: {}", self.0);
// }
// }

// There is no need to declare lifetimes of references to &self or &mut self,
// as they have the reference of their implementation and are inferred by the compiler
impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

// Here we do need the lifetimes, because we want to ensure that all lifetimes `a outlive the
// lifetime of their struct, Person.
//
// That said, there is an easier way to do this using the `static lifetime annotation
// struct Person<'a> {
// age: u8,
// name: &'a str,
// }

struct Person {
    age: u8,
    name: &'static str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    println!("Success!");
}
