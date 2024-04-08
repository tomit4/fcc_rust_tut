// The `&self1` is actually short for `self: &Self`. Within an
// impl block, the type Self is an alias for the type that the
// impl block is for. Methods must have a parameter named self
// of type Self for their first parameter, so Rust lets you abbreviate
// this with only the name self in the first parameter spot.

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
