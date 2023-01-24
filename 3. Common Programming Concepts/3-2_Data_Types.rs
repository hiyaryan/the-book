fn main() {
    // Rust is a statically typed language--must know the types of all variables at compile time.
    // Compiler can infer the type based on the value and how its used.
    let x = 42; // u32 is inferred
    println!("x = {x} has an inferred type of u32.");
    
    // If mutltiple types are possible, a type annotation is required
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("You guess {guess}.");

    // Two data type subsets in Rust are scalar and compound.
    
    // Scalar--represents a single value
    // Primary scalar types: integers, floating-point numbers, Booleans, characters.

    
    // Integer Types
    
    // length:   8-bit 16-bit 32-bit 64-bit 128-bit arch
    // signed:   i8    i16    i32    i64    i128    isize
    // unsinged: u8    u16    u32    u64    u128    usize
    
    // arch 'architecture': depends on the computer architecture running the program (e.g. 32-bit or 64-bit)

    // number literals
    // decimal    hex    octal    binary       byte(u8 only)
    // 98_222     0xff   0o77     0b1111_0000  b'A'
    
    // Note that number literals can be written with _ to improve readbility

    // default integer type: i32
    // isize and usize are used primarily for indexing some sort of collection

    // integer overflow: occurs when there is a size and type mismatch--produces an Unrecoverable Error with 'panic!'

    // Floating-point Types
    // Two primitive types all signed: f32 and f64
    // Represented according to the IEEE-754 standard
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder 'modulo'
    let remainder = 43 % 5;


    // Boolean Type
    // two values: true or false
    let t = true;
    let f: bool = false; // with explicit type annotation


    // Character Type
    // 
    let c = 'z'; 
    let z: char = 'ℤ'; // with explicit annotation
    let heart_eyed_cat = '😻';


    // Compound Types
    // Groups multiple values into one type.
    // Two compound types: 1. Tuples, 2. Arrays.

    // 1. Tuples
    // - Tuples have a fixed length.
    // - Created with a comma separated-list of values inside parenthesis.
    // - Each position in the tuple may be represented with any type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Use pattern matching to destructure a tuple value.
    // Below destructures the tuple into 3 separate values x, y, z.
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // Access a tuple element directly by using a period '.'i
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Tuples with no values are called 'unit' tuples.
    // Unit values are implicitly returned if no other value is returned.
    let unit: () = ();

    // 2. Arrays
    // - A collection of of values of the same type.
    // - Arrays have a fixed length.
    // - Arrays are written as a comma-separated list inside square brackets.
    let a = [1, 2, 3, 4, 5];
    
    // - Arrays store data on the stack rather than the heap.
    // - Arrays ensure a fixed number of elements which make them most useful.
    // - A vector is a similar collection type that can grow or shrink and is more commonly used (see ch 8).
    let months = ["January", "February", "...", "December"];

    // Write an arrays type and size:
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize an array with the same value:
    let a = [3; 5];

    // - Arrays are single chunks of memory of a known, fixed size on the stack.
    // To access an element in an array:
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // ERROR: Index out of bounds.
    // If an element that is past the end of the array is attempted to be accessed the runtime
    // error above with be thrown.
    // This kind of error is a safety principle of Rust; many low level languages do not protect against
    // incorrect indexing and instead returns invalid data.
   
}
