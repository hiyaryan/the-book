// Method Syntax
// Methods are similar to functions. They are declared with `fn`, have parameters and return values, and contains code that is
//    run when the method is called. The difference, is that methods are defined in context of a struct (or an enum-ch 6, or trait
//    object-ch 17) and that their first parameter is always `self` representing the instance of the struct the method is 
//    being called on.

// Defining methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define a method inside the `impl` block of a struct
// the impl block will be associated with the defined struct
impl Rectangle {
    // The first parameter in every method should be &self (shorthand for self: &Self))
    // Self is an alias for the type that the impl block is for, `&` must be used in in shorthand form, &self,
    //     to indicate the method borrows the Self instance. Methods can take ownership of `self` so this makes it
    //     so that self is borrowed immutably. To change the instance the method was called on, use `&mut self`
    //     as the first parameter. (it is rare to have a method that takes ownership of self, its usually used
    //     to tranform self into something else preventing the caller from using the original instane).
    // Use methods instead of functions to provide method syntax, not having to repeat `self` in every method
    // signature, and for organization.
    //     All the things that can be done on an paricular instance can be organized into one `impl` block.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Rust knows the difference between width() and width, method and field, in a struct.
    // Generally, methods with the same name as a field in a struct only return the value, these are called getters.
    //     This allows the field to be made private, and read-only, through the getter method, as part of the
    //     type's public API. (See Chapter 7 for more info on private and public methods)
    if rect1.width() {.
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}