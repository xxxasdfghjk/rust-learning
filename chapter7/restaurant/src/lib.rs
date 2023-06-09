use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_paymjjjjent() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::hosting::add_to_waitlist();
    }
    fn cook_order() {}
}
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //絶対パス
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    meal.toast = String::from("wheat");
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("I'd like {} toast please", meal.toast);
}
