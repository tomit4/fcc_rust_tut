/*
 * Reprensentation
 *
 * A String is made up of three components: a pointer to some bytes, a length and a capacity.
 *
 * The pointer points to an internal buffer String uses to store its data. The length is the number
 * of bytes currently stored in the buffer (always stored on the heap), and the capacity is the
 * size of the buffer in bytes. As such, the length will always be less than or equal to the
 * capacity.
*/

// If a String has enough capacity, adding elements to it will not re-allocate
// Modify the code below to print out:
// 25
// 25
// 25
// Here, there's no need to allocate more memory inside the loop.

fn main() {
    // let mut s = String::new(); // defaults to allocating 0 bytes
    let mut s: String = String::with_capacity(25); // with_capacity allows for specifying how many
                                                   // bytes we wish to initially allocate

    println!("{}", s.capacity());

    for _ in 0..2 {
        // s.push_str("hello"); // defaults to 8 bytes, then 16 bytes if used with ::new()
        s.push_str("hellohellohellohello"); // defaults to 25 if used with with_capacity(25),
                                            // doubles to 50 on second iteration as it exceeds
                                            // capacity
                                            // larger allocations can make your program faster as
                                            // the compiler does not need to re-allocate more
                                            // memory as often (however, this also means there can
                                            // be more memory allocated each time which can take up
                                            // more memory overall)
        println!("{}", s.capacity());
    }

    println!("Success!");
}
