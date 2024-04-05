// Each enum variant can hold its own data.

// Fill in the blank
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1: Message = Message::Move { x: 1, y: 2 }; // Instantiating with x = 1, y = 2
    let msg2: Message = Message::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
}
