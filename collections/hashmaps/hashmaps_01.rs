// FILL in the blanks and FIX the errors
use std::collections::HashMap;
fn main() {
    // Instantiates an empty hashmap
    let mut scores: HashMap<&str, i32> = HashMap::new();
    // Inserts key/value pairs into hashmap
    // NOTE: keys must be of same type (i.e. strings here)
    // NOTE: values must be of some type (i.e. i32 here)
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    // scores.insert("Ashley", 69.0); // error, floating point instead of i32
    scores.insert("Ashley", 69);
    // scores.insert("Katie", "58"); // error, String instead of i32
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score: i32 = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}
