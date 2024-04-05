// Fix the errors without adding or removing lines
fn main() {
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
    // because for is a function, it would take ownership
    // of `names` if we didn't reference it using `&`
    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        println!("{}", n);
    }

    println!("{:?}", numbers);
}
