/* Make it work */
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() {
        // .iter() doesn't take ownership
        println!("{}", i);
    }

    println!("{:?}", arr);
}
