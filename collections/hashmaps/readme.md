## HashMaps

- HashMaps are a type of data structure to store key-value pairs

-HashMaps are allocated on the heap as it is dynamically sized, and can grow and
shrink

- Hashmaps allow for efficient lookups, insertions, and deletions of data

- Each key is hashed to a unique index in an underlying array

### HashMap (from practice.rs)

Where vectors store values by an integer index, HashMaps store values by key. It
is a hash map implemented with quadratic probing and SIMD lookup. By default,
`HashMap` uses a hashing algorithm selected to provide resistance against
HashDoS attacks (otherwise known as a Collision attack).

The default hashing algorithm is currently `SipHash 1-3`, though this is subject
to change at any point in the future. While its performance is very competitive
for medium size keys, other hashing algorithms will outperform it for small keys
such as integers as well as large keys such as long strings, though those
algorithms will typically not protect against attacks such as HashDoS.

The hash table implementation is a Rust port of Google's [SwissTable](https://abseil.io/blog/20180927-swisstables). The original C++ version of SwissTable can be found [here](https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h), and this [CppCon talk](https://www.youtube.com/watch?v=ncHmEUmJZf4) gives an overview of how the algorithm works.

### Requirements of HashMap key

Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. This includes:

- `bool` (though not very useful since there is only two possible keys)
- `int`, `uint`, and all variations thereof
- `String` and `&str` (tips: you can have a `HashMap` keyed by `String` and call `.get()` with an `&str`)

Note that `f32` and `f64` do not implement `Hash`, likely because [floating-point precision](https://en.wikipedia.org/wiki/Floating-point_arithmetic#Accuracy_problems) errors would make using them as hashmap keys horribly error-prone.

All collection classes implement `Eq` and `Hash` if their contained type also respectively implements `Eq` and `Hash`. For example, `Vec<T>` will implement `Hash` if `T` implements `Hash`.

(see ./hashmaps_04.rs)

### Capacity

Like vectors, HashMaps are growable, but HashMaps can also shrink themselves when they have excess space. You can create a HashMap with a certain starting capacity using HashMap::with_capacity(uint), or use HashMap::new() to get a HashMap with a default initial capacity (recommended).

**Example**

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // Indeed ,the capacity of HashMap is not 100, so we can't compare the equality here.
    assert!(map.capacity() >= 100);

    // Shrinks the capacity of the map with a lower limit. It will drop
    // down no lower than the supplied limit while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    // Shrinks the capacity of the map as much as possible. It will drop
    // down as much as possible while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    println!("Success!");
}
```

### Ownership

For types that implement the Copy trait, like i32 , the values are copied into HashMap. For owned values like String, the values will be moved and HashMap will be the owner of those values.

(see ./hashmaps_05.rs)
