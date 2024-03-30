## Writing and Running a Rust Program

Next, make a new source file and call it main.rs. Rust files always end with the .rs extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use hello_world.rs rather than helloworld.rs.

Now open the main.rs file you just created and enter the code in Listing 1-1.

Filename: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Listing 1-1: A program that prints Hello, world!

Save the file and go back to your terminal window in the ~/projects/hello_world directory. On Linux or macOS, enter the following commands to compile and run the file:

```sh
rustc main.rs
./main
# Hello, world!
```
