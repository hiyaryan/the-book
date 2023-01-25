// Enums and Pattern Matching
// Enums == Enumerations
// These define a type by enumerating its possible variants.

// Defining an Enum
// Structs allow grouping of related fields and data.
// Enums allow relating one value as part of a possible set of values.
//    E.g. Rectangle is part of a set of possible shapes indluding Circle and Triangle
//    Rust allows the encoding of these possibities as an enum

// When are enums more appropriate than structs? 
// e.g. When IP addresses need to be enumerated over two possible variants: v4 and v6

// IpAddrKind is a custom data type that can be used elsewhere in the code.
// All this tells us is what kind of data these variables store. 
// How do we store the IP address data?
// enum IpAddrKind {
//     V4,
//     V6,
// }

// Enum data can be put into a Struct
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// }

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1")
// }

// More concisely, data can be put directly into the enum.
// In this case each IpAddrKind is associated with String types
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1"));

// Another advantage for using enums is the ability for each kind in the enum
// to be assiciated with different types.
// In this case one IpAddrKind is associated with u8 and the other a String
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The standard library also has struct definitions for these kinds of IP addresses
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// As long as the IpAddr has not been imported into scope of this file, there is no conflict with
// defining a custom IpAddr separate it.

// The main point is that any kind of data can be put inside an enum variant.
// Standard Library types are not much more complicated.

// Enum example with a wide variety of types
#[derive(Debug)]
enum Message {
    Quit, // no associated data
    Move {x: i32, y: i32}, // named fields like a struct
    Write(String), // a single string
    ChangeColor(i32, i32, i32), // three i32 values
}

// Above can be written as different struct definitions
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Issue with defining all of the variants of message in there own struct is that a single function
// cannot be easily defined to take any kind of these messages (4 different functions would be needed or
// a function with 4 parameters).

// Methods can also be defined on enums
impl Message {
    fn call(&self) {
        // do stuff with message
    }
}

// The Option Enum and Its Advantage Over Null Values
// The `Option` enum type encodes in it the scenario of a value being something or nothing. 
// Scenario 1: Request of a list containing items returns something
// Scenario 2: Request of a list containing nothing returns nothing-this is a common bug in other languages
// Rust excludes the null value in place of the Option enum which is defined by the standard library as follows
enum Option<T> {
    None,
    Some(T),
}

// The Option type is included in the prelude (it does not need to be brought into scope explicitly)
// It's variants: Some and None, are also in the prelude, so they can be used without the `Option::` prefix.
// Option<T> is still a regular enum, with Some(T) and None as its variants.
// The <T> syntax represents a generic type (see chapter 10 for more info on generic types)
//     Generic types can hold one piece of data of any type.

// In order to use an Option<T> value, you want code that will handle each variant Some and None.
// The `match` expression can be used to handle variants in enums.

fn main() {
    // Enum Values
    // The variants of the enum are namespaced under its identifier separated by `::`
    // The following variants are of the same type: IpAddrKind
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // Call the route function with either variant
    // route(four);

    // Assigning different kinds of IpAddr 
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // Using a method defined in an enum.
    let m = Message::Write(String::from("hello"));
    m.call();

    // Examples using the Option type-the data types are inferred.
    // The option variant, Some or None, must be explicitly annotated. 
    let some_number = Some(5); // Option<i32>
    let some_char = Some('e'); // Option<char>
    let absent_number: Option<i32> = None; // This is the same thing as null kind of..

    // ..None is better than null because Option<T> and T are different types. The compiler won't let
    // Option<T> be used as if it were an actual valid value.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; // This won't compile because it's trying to add two different types.
    // Option<T> must be converted to T before opertations can be performed.
    // Using the Option<T> enum requires you to explicitly handle the case when the value is null.
    // Everywhere where Option<T> is not a value can be safely assumed that the value is not null.

    // To get T out of Option<T> checkout the wide number of methods in its documentation that can.
}

// Define a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {

}