// Implement struct Point to make it work.

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let _integer: Point<i32> = Point { x: 5, y: 10 };
    let _float: Point<f64> = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
