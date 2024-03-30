// Make it work in two distinct ways

fn main() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // 0.1 + 0.2 = 0.30000000000002
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32); // 0.1 + 0.2 = 0.30000000000002

    println!("Success!");
}
