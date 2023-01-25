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

    // This function checks if a smaller rectangle can be held inside this retangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    // All functions defined within an impl block are called associated functions because they are associated with
    // with the type named after impl.
    // Associated functions can be defined without `self` as the first parameter (these are not methods), because they
    // don't need an instance of the type to work with.
    // Associated functions that aren't methods are used for constructors (returns a new instance of the struct) these are
    // equivalent to using the keyword `new` in other languages.

    // The following function returns a rectangle with equal width and height.
    // Self keyword in the return type and in the body of the function are aliases for the type that appears after the impl
    // keyword (in this case that is the Rectangle type).
    // Call this associated function using the `::` syntax.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Automatic Referencing and Dereferencing
// Unlike C/C++, Rust doesn't have an equivalent `->` operator. Methods are one of the few places in Rust
// that have this behavior.
// How it works
// When a method is called with `object.something()`, `&`, `&mut`, or `*`, are automatically added so that
// object automatically matches the signature method.
//     E.g. The following are the same:
//          p1.distance(&p2);
//          (&p1).distance(&p2);

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Rust knows the difference between width() and width, method and field, in a struct.
    // Generally, methods with the same name as a field in a struct only return the value, these are called getters.
    //     This allows the field to be made private, and read-only, through the getter method, as part of the
    //     type's public API. (See Chapter 7 for more info on private and public methods)
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // By using `&`, `main` retains ownership of the objects, effectively making it read-only.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling the constructor in Rectangle that constructs a square
    let sq = Rectangle::square(3);
    println!("A retangle with width {} and height {} is called a square.", sq.width, sq.height);
}
