// A Example Program Using Structs
// This example calculates the area of a rectangle using structs. The example begins first with
// single variables, then is refactored into structs.

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