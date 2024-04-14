// Make a constant with `'static` lifetime.
static NUM: i32 = 18; // memory location remains static, as opposed to:

// const NUM: i32 = 18; // memory location can change depending on what scope it is passed to

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    // 'static has a greater lifetime than `a
    // but here we coerce the lifetime of &NUM (i.e. `static) to `a
    &NUM
}

fn main() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num: i32 = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static: &i32 = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

// Output:
// coerced_static: 18
// NUM: 18 stays accessible!
