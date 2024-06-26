/* Make it work by adding proper lifetime annotation */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x: &str = "long";
    let y: &str = "longer";

    println!("{}", longest(x, y));
}
