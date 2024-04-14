#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    Some(&mut Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })
}

fn main() {
    // NOTE: because of use of unsafe, the tutorial series does not cover it as it is more advanced
    // than is within the scope of an introductory course
    unsafe {
        config = init();

        println!("{:?}", config)
    }
}
