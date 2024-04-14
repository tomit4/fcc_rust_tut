/* Make it work */

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn main() {
    /* 'a tied to fn-main stackframe */
    let var_a: u32 = 35;
    let example: Example;

    {
        /* Lifetime 'b tied to new stackframe/scope */
        let var_b: NoCopyType = NoCopyType {};

        /* fixme */
        example = Example {
            a: &var_a,
            b: &var_b,
        };
        // this works because &var_b is still alive
        println!("(Success!) {:?}", example);
    }

    // Cannot compile because example now holds a reference to `var_b`,
    // which only lives within the inner scope instantiated above, this
    // is a dangling reference/pointer
    // To get this to work, you either put the println!() statement in the inner scope, OR
    // remove the inner scope alltogether so all variables live within the main() scope
    // println!("(Success!) {:?}", example);
}
