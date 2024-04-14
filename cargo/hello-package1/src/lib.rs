pub mod front_of_house;
// Moved into front_of_house dir when organizing modules
// mod front_of_house {
// pub mod hosting {
// pub fn add_to_waitlist() {}
// pub fn seat_at_table() {}
// }
// pub mod serving {
// pub fn take_order() {}
// pub fn serve_order() {}
// pub fn take_payment() {}
// pub fn complain() {}
// }
// }

mod back_of_house;
// Moved into back_of_house.rs when organizing modules
// mod back_of_house {
// fn fix_incorrect_order() {
// cook_order();
// // FILL in the blank in three ways
// // 1. using keyword 'super'
// // 2. using absolute path
// // super::front_of_house::serving::serve_order();
// crate::front_of_house::serving::serve_order();
// }

// fn cook_order() {}
// }

pub fn eat_at_restaurant() -> String {
    // Call add_to_waitlist **absolute path**:
    // crate::front_of_house::hosting::add_to_waitlist();
    // Call add_to_waitlist **relative path**:
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
