mod front_of_house {
    // has to be public to be seen by front_of_house
    pub mod hosting {
        // has to be public to be seen by hosting
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    // public struct
    pub struct Breakfast {
        // public struct field
        pub toast: String,
        // private struct field
        seasonal_fruit: String,
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
        // super is same as ../
        super::serve_order();
    }

    fn cook_order() {}
}

// can only see front_of_house since it is a sibling
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change toast field
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this will not work since seasonal_fruit is private
//    meal.seasonal_fruit = String::from("blueberries");
}

// bring hosting into scope (use) and re-export to above scope (pub)
pub use crate::front_of_house::hosting;

// provide alias
use crate::front_of_house::hosting::add_to_waitlist as host_waitlist;

pub fn also_eat_at_restaurant() {
    hosting::add_to_waitlist();
    // use alias
    host_waitlist();
}

fn main() {}
