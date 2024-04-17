/* Make it work with least amount of changes*/
fn main() {
    let color: String = String::from("green");

    // let print = move || println!("`color`: {}", color);
    // Remove the `move` keyword to allow repeated borrowed
    let print = || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow: &String = &color;

    println!("{}", color);
}
