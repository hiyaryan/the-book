// A Example Program Using Structs
// This example calculates the area of a rectangle using structs. The example begins first with
// single variables, then is refactored into structs.

fn main() {
    // With single variables
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}