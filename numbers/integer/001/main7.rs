// Floating Point
fn main() {
    let _x: f64 = 1_000.000_1; // ? => f64
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of(&_x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
