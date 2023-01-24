fn main() {
    // ref: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    // Variables are immutable by default. Add keyword 'mut' to make is mutable.
    let mut x = 5;
    println!("The value of x is {x}.");
    x = 6;
    println!("The value of x is {x}.");

    // Constants are always immutable and cannot be used with the key word 'mut'.
    // Constant evaluation: https://doc.rust-lang.org/reference/const_eval.html
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    // Shadowing
    // Decalaring a variable with the same name is called shadowing.
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is {y}.");

    // Using let allows us to change the type that the variable is assigned to.
    let spaces = "    ";
    println!("You entered {spaces}.");

    let spaces = spaces.len();
    println!("The number of spaces is {spaces}.");
    // The following produces an error because 'mut' variables must have the same type.
    // let mut spaces = "    ";
    // spaces = spaces.len();
}

