mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Best practice: idomatic way to open a module's functions.
// Adding `pub` makes it so it is callable by other parts of the code outside the scope.
pub use crate::front_of_house::hosting;

// Not a good idea to follow the path all the way to a specific declaration within a module
// use crate::front_of_house::hosting::add_to_waitlist;

// Use makes it so that a modules functions can be used in other parts of the code.
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Use `super` to access the another modules modules functions outside of the scope of this module or
// add the use statement into the module itself.
mod customer {
    pub fn eat_at_restaurant() {
        // It is clear where this function is coming from.
        super::hosting::add_to_waitlist();
    }

    // pub fn eat_at_restaurant_2() {
        // It's unclear where this function is coming from?
        // super::add_to_waitlist();
    // }
}

