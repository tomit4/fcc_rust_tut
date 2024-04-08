// We can also use the impl Trait syntax in the return position
// to return a value of some type that implements a trait.
// However, you can only use impl Trait if you’re returning a
// single type, use Trait Objects instead when you really need
// to return several types.

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use trait object.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal: Box<dyn Animal> = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
