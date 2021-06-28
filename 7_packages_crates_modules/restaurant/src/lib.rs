mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    // In a public struct, each public field must be labelled.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from("toast"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If we make an Enum public, all variants are public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about the type of bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // Cannot modify the private seasonal_fruit field
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting;
// Relative: use self::front_of_house::hosting

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

// By re-exporting, external code can call add_to_waitlist using
// hosting::add_to_waitlist
// pub use crate::front_of_house::hosting;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
