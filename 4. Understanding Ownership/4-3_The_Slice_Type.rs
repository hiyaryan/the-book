// The Slice Type
//
// Slices reference a contiguous sequence of elements in a collection rather than the whole
// collection.
// Slices are a kind of reference so it does not have ownership.
//
// Syntax: &String_Variable_Name[starting_index..ending_index]
// &String_Variable_Name[0..2] OR &String_Variable_Name[..2] slicing 0 to 2.
// &String_Variable_Name[3..len] OR &String_Variable_Name[3..] slicing 3 to end
// &String_Variable_Name[0..len] OR String_Variable_Name[..] slice the entire string.
//
// Note: String slice range indices must occur at valid UTF-8 character bounderies.
//
// Summary:
// Ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
// Having an owner responsible for automatically cleaning up data when the owner goes out of scope
// prevents additional code and debugging.
//
// Note: Ownership affects how many other parts of Rust works.

fn main() {
    // Part of first_word function example not using String Slices
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // empties String, setting it to ""
    // word is still 5 here with no string that can meaningfully use it.

     println!("First word is: {}", word);


    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5]; // hello is a reference to a portion of the string
    let world = &s[6..11]; // same with world

    let w1 = first_word_with_slices(&s);
    println!("First word is: {}", w1);


    // String literals are slices
    let s2 = "Hello, world!"; // The type of this string is: &str, a slice pointing to that specific point of the binary.

    // String literals are immutable because &str is an immutable reference.
    let w2 = first_word_with_slices(s2);
    println!("First word is: {}", w2);


    // Examples slicing strings and literals
    let my_string = String::from("hello world");
    let word = first_word_with_slices(&my_string[0..6]); // partial slices
    let word = first_word_with_slices(&my_string[..]); // whole slices
    let word = first_word_with_slices(&my_string); // equal to whole slice
   
    let my_string_literal = "hello world";
    let word = first_word_with_slices(&my_string_literal[0..6]); // partial slice of string literal
    let word = first_word_with_slices(&my_string_literal[..]); // whole slice of string literal
    let word = first_word_with_slices(my_string_literal); // equivalent to whole slice of string literal


    // Other Slices
    // More general slices exist for other types
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // has type &[i32]

    // Works the same as string slices do:
    // Stores a reference to the first element and a length.
    // Can be used for all sorts of collections (ch 8)

    assert_eq!(slice, &[2, 3]);
}
// Example attempting to demonstrate why String slices are better...
// Without using slices, this function takes a string of words separated by spaces. Returns the first word in the string.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Note. iter(): returns each element in a collection. 
    //       enumerate(): wraps the result of iter returning each element as part of a tuple.
    // First element of the tuple is the index, second element is a reference to the element. (more
    // convenient than calculating the tuple yourself.)
    // Patterns can be used to destructure the tuple: 
    // The pattern has `i` for the index in the tuple and &item for the single byte in the tuple.
    //
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }  
    }

    s.len()
}

// Example with slices
// fn first_word_with_slices(s: &String) -> &str {
// Knowing the type of a string literal is a reference, the function signature can be written as
// follows allowing &String and string literals to be passed to it.
// This takes advantage of deref coercions (ch 15). 
fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
