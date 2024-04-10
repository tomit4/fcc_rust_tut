// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3]);

    for i in 0..5 {
        // println!("{:?}", v[i]) // causes out of bounds compiler panic
        println!("{:?}", v.get(i)) // instead uses an Option<i32>
                                   // Prints:
                                   // Some(1)
                                   // Some(2)
                                   // Some(3)
                                   // None
                                   // None
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2),
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
