// You can use utf8_slice to slice UTF8 string, it can index chars instead of bytes.

use utf8_slice; // no utf8_slice module in root, must use cargo to install
fn main() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // Will equal "🚀"
}
