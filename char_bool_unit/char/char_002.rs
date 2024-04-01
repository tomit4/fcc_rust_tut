// make it work
fn main() {
    let c1: char = '中';
    // this fails because double quotes are for strings, and single quotes are
    // recognized as characters
    // let c1: char = "中";
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
