// The `for in` construct can be used to iterate through an Iterator, e.g a range a..b.

fn main() {
    for n in 1..100 {
        // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}
