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
enum IpAddrKind {
    V4,
    V6,
}

fn main() {

}