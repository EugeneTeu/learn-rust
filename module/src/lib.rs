mod front_of_house {
    pub mod hosting {

        pub fn add_to_waitlist() {}
    }
    mod serving {

        fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
