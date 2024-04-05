fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        // note the use of `i + 1` here
        println!("The {}th element is {}", i + 1, v);
    }
}
