// A Example Program Using Structs
// This example calculates the area of a rectangle using structs. The example begins first with
// single variables, then is refactored into structs.
#[derive(Debug)] // Add this outer attribute to the top of a struct to opt-in to debugging 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // With single variables
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // Make a single variable somewhat more readable by refactoring it...
    // With tuples
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area_tuples(rect1));

    // Make a tuple even more readable by refactoring it...
    // With structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area_structs(&rect2));

    // Adding useful functionality with derived traits
    // Note on println! macro:
    //     {} tell it to use formatting known as "Display": output intended for direct end user consumption.
    //     With structs, there are multiple ways to display it: with commas, in curly brackets, with all fields (rust will not guess which)
    //     The following helpful error displays for the next line of code
    //         = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    //         = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // println!("rect2 is {}", rect2);

    // The following line produces this error if no #[derive(Debug)] outer attribute is added to the top of the struct
    //     = help: the trait `Debug` is not implemented for `Rectangle`
    //     = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
    // In order to use debugging information, you must explicitly opt in to make the functionality available for the struct.
    // Pretty print using: {:#?} or on a single line with {:?}
    println!("rect2 is {:#?}", rect2);
    println!("rect2 is {:?}", rect2);

    // Use the dbg! macro instead of println! to print to the standard error console stream (stderr)
    // See ch 12 for more information on the stderr
    // For integers debug formatting prints only their value
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale), // since dbg! returns ownership of the expressions value the width field will get the same value as if dbg! wasn't there
        height: 50,
    };

    // For structs the debug formatting prints it in pretty format
    dbg!(&rect4); // Careful not to let dbg! take control of rect4, use a reference to rect4 instead so it may be used again later.

    // See Chapter 10 for how to use the `derive` attribute that may be useful to add onto custom types
    // Traits and behaviors are listed in Appendix C

    // See how to add other attributes to rust code in the "Attributes" section of the Rust Reference.
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}