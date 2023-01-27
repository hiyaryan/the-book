mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

// Use makes it so that a modules functions can be used in other parts of the code.
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Use `super` to access the another modules modules functions outside of the scope of this module or
// add the use statement into the module itself.
mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}