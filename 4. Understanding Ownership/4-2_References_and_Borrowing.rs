// References and Borrowing
// Reference is like a pointer to an address that can be followed to access data stored at the
// address which is owned by some other variable.
//
// A reference is guaranteed to point to a valid value of a particular type for the life of the
// reference.
//
// The actions of creating a reference is called borrowing.
// Trying to modify something that is being borrowed doesn't work.
//
//  The opposite of referencing `&` a string is dereferencing `*`.
//
// Restriction to mutable references: Mutable references can have no other references to that
// value.
//
// This restriction allows mutation in a controlled fashion.
// ! This is a common pain point for new rust programmers.
// The main purpose of this restriction is to prevent data races at compile time.
//
// Data Race is similar to a race condition and occurs when:
// 1. Two or more pointers access the same data at the same time.
// 2. At least one of the pointers is being used to write to the data.
// 3. There's no mechanism being used to synchronize access to the data.
//
// Data races cause undefined behavior and are difficult to fix. Rust restricts mutable references
// to a single variable to prevent this from happening.
//
// Non-Lexical Lifetimes (NLL): the ability of the compiler to tell that a reference is no longer
// being used at a point before the end of the scope.
//
// Dangling Pointer:
// a pointer that references a location in memory that may have been given to someone else
// freeing memory while preserving a pointer to that memory
// In Rust, the compiler ensures that data will not go out of scope before the reference to the
// data does.
//
//
// The Rules of References
// - At any given time you can have only one mutable reference 
// - You can have any number of immutable references
// - References must always be valid.


fn main () {
    // Creating a string in the heap
    let s1 = String::from("hello");

    // `&` sign represents references
    // They allow you to refer to some value without taking ownership of it.

    // Access the string via reference.
    // `&s1` called a reference, it does not own s1, which entails that it will be dropped once the
    // reference stops using it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    // Passing a mutable reference.
    change(&mut s);

    // Mutable references restriction example:
    // This code fails because there can only be one variable referencing a mutable reference.
    // let r1 = &mut s;
    // let r2 = &mut s;

    // print!("{}, {}", r1, r2)
    
    {
        let r1 = &mut s;
    } // r1 goes out of scope here
    
    // r2 can be made since r1 is out of scope.
    let r2 = &mut s;


    // A similar restriction is made when combining mutable and immutable references.
    let mut s3 = String::from("hello");
    
    // let r1 = &s; // no problem
    // let r2 = &s; // np problem
    // let r3 = &mut s; // PROBLEM

    // print("{}, {}, and {}", r1, r2, r3);
    
    // Reference Scope
    // The scope of a reference starts where it is introduced and continues through the last time
    // that the reference is used.
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // variables r1 and r2 will not be used after this point.
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

    // How to define and use a function that has a reference to an object as a parameter instead of
    // taking ownership.
    // The function signature indicates that the type of parameter is a reference using `&`.
fn calculate_length(s: &String) -> usize {
    
    s.len()

    // s.push_str(" world!") // Error: Cannot mutate a variable behind &.
} // `s` goes out of scope, but since the function does not have ownership of it it is not dropped.

// Adding `mut` after `&` makes the reference mutable. 
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}


// Dangling References
// fn dangle() -> &String { // returns a reference to a String
    // let s = String::from("hello"); // s is a new String

    // This produces an error
    // No value to be borrowed from, consider using the `static` lifetime (see chapter 10)
     // &s // return a reference to the String, s
// } `s` goes out of scope and is dropped, its memory goes away. Danger!

// Solution to dangle
fn no_dangle() -> String {
    let s = String::from("Hello!");

    s
}
