// 1. Functions

// main is the entry point to a Rust program.
// New functions can be defined before or after main.
fn main() {
    println!("Hello, world!");

    // Call a function using its name followed by parenthesis.
    another_function();

    a_function_with_parameters(5);

    print_labeled_measurement(5, 'h');

    expressions();
    
    // The follwing line shows a return value is expected. It is the same as "let x = 5;"
    let x = five();
    println!("The numerical value of five is {x}.");

    let x = plus_one(5);
    println!("5 + 1 = {x}.");
}

// fn is a keyword used to declare a new function.
// Function names are written in snake_case.
// Curly brackets tell the compiler where the function body begins and ends.
fn another_function() {
    println!("Another function.");
}


// 2. Parameters

// Functions can be defined with parameters.
// Parameters are special variables that are part of a function's signature.
// Arguments can be passed to a function that are defined with parameters.
// Below, x is the parameter of type i32, which must be declared in a functions signature.
// Requiring a type annotation means it is never needed in the code elsewhere. 
fn a_function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// Define a function with two parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


// 3. Statements and Expressions
// Rust is an expression based language.
// Function bodies are a series of statements optionally ending in an expression.

// def. Statements - instructions that perform some action and do not return a value.
// def. Expressions - evaluate to a resultant value.

// A function definition is a statement.
fn statements() {
    // Creating a variable and assigning it a value with the let keyword is a statement.
    let y = 6;

    // Statements do not return values.
    // let x = (let y = 6); // Error. 
    
    // Since Rust does not allow statements to return a value the following is invalid
    // x = y = 6 // Error.
}

// Expressions evaluate to a value making up most of the rest of the code to be written.
// Math operations are expressions.
// A single value is an expression (e.g. y = 6 is a statement with a single value 6 as an
// expression).
// 
fn expressions() {
    // "let y" is a statement.
    // "{...}" the braces and everything within are part of the expression.
    let y = {
        let x = 3;
        x + 1 // This line does not end in a ';' because it "returns" a value.
              // adding a ';' turns it into a statement and does not return a value
    };

    println!("The value of y is: {y}");
}


// 4. Functions with Return Values
// Return values are declared in the function definition with an '->'
// The return value is synonymous with the final expression in the block of the function body.
fn five () -> i32 {
    5
}

// This function accepts an argument passed to the x parameter that then returns it plus 1.
fn plus_one(x: i32) -> i32 {
    // A semicolon will will result in an error because the function is expecting a return
    // expression but received a statement.
    // The function is expecting a return type of i32 but received a return type of (), this is
    // because a statement evaluates to the unit type () -> type mismatch.
    x + 1
}
