/* Make it work by adding proper lifetime annotations */
/*
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Note here that the lifetime here is annotated as '_'
// This is because we don't need to reference the lifetime of the struct,
// we only need to annotate the lifetime of the function
impl ImportantExcerpt<'_> {
    fn level<'a>(&'a self) -> i32 {
        3
    }
}
*/
// The above works, but there is a more simple way,
// we can simply use the 'static lifetime to tell
// the compiler we wish the reference, `&str` to
// live the entire life of the program
struct ImportantExcerpt {
    part: &'static str,
}

// Note here that the lifetime here is annotated as '_'
// This is because we don't need to reference the lifetime of the struct,
// we only need to annotate the lifetime of the function
impl ImportantExcerpt {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    println!("Success!");
}
