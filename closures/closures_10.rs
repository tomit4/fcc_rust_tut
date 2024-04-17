/* Fill in the blank using two approaches,
and fix the error */
// fn create_fn() -> impl Fn(i32) -> i32 {
// you can also isntead of an impl, simply Box the return
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num: i32 = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    // A: add move to make sure num's ownership is moved outside the scope of this function
    // move |x| x + num
    // A: You can also assign this return to a new place in memory
    Box::new(move |x| x + num)
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
