/* Fill in the blank in two ways */
fn main() {
    // let v = "hello";
    // let v: &'static str = "hello";
    let v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}
