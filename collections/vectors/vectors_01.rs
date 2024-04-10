fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    // NOTE: When converting an array to a vector, make sure to use `::from`
    // as otherwise using the vec![] will simply create a vector with an array inside of it
    // i.e. a vector with an array nested inside
    let v: Vec<u8> = Vec::from(arr);
    is_vec(&v);

    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    let mut v1: Vec<u8> = Vec::new();
    is_vec(&v1);

    // NOTE: the small example of referencing and dereferencing here
    // very simply, we don't want to take ownership of v here, so we use a reference `&` to v
    // but because we do this, we then have to dereference it `*` in the loop itself
    for i in &v {
        v1.push(*i);
    }

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}
