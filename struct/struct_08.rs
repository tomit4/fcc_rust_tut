// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name: String = f.name;

    // ONLY modify this line
    println!("{}, {}", _name, f.data);
}
