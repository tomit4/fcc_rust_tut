/* Fill in the blank and fix the error*/
/* A: Because the compiler can't know which of the conditions will execute, we must dynamically
 * dispatch the result by Boxing the return*/
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        // move |x| x + num
        Box::new(move |x| x + num)
    } else {
        // move |x| x + num
        Box::new(move |x| x + num)
    }
}

fn main() {}
