// Fix the error without removing code line
fn main() {
    let mut s: String = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s);
}
