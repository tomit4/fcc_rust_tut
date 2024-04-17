/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count: i32 = 0;

    // add `move` keyword to have the closure take ownership of the count value
    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    let _reborrow: &i32 = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}
