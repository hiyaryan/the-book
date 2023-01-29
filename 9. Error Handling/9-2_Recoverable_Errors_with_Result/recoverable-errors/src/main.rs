use std::fs::File; // Return type of File::open is a Result<T, E>
                   // use std::io::ErrorKind;
use std::io::{self, Read};

use std::fs;
// use std::io;
use std::error::Error;

// Box<dyn Error> is a trait object (see ch 17 for more information)
// It can be read as meaning "any kind of error".
// Using it makes it so that any new code added using ? to the main function will still
// be valid as a catch all.
// If main returns a Result, it will exit with a value of 0 if main returns Ok(()) or it will exit
// with a nonzero value if it returns an Err value.
fn main() -> Result<(), Box<dyn Error>> {
    // let greeting_file_result = File::open("hello.txt");

    // If the result is Ok, the code will return the inner file value out of the Ok variant
    // If there is no file named hello.txt, the error will be seen out of the panic! macro.
    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening file: {:?}", error),
    //};

    // Catch different kinds of failures
    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //        ErrorKind::NotFound => match File::create("hello.txt") {
    //            Ok(fc) => fc,
    //           Err(e) => panic!("Problem creating the file: {:?}", e),
    //        },
    //        other_error => {
    //            panic!("Problem opening the file {:?}", other_error);
    //        }
    //    },
    //};

    // Catch different kinds of failures with Result<T, E>
    //let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //           panic!("Problem creating the file: {:?}", error);
    //        })
    //    } else {
    //        panic!("Problem opening the file {:?}", error);
    //    }
    //});

    // Catch error using using the `unwrap` method from Result<T, E>
    // let unwrap_greeting_file = File::open("hello.txt").unwrap();

    // Catch error using using the `expect` method from Result<T, E>
    let expect_greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propogating Errors
    let username = read_username_from_file();

    // Propagating Errors more concisely
    let username = read_username_from_file_concisely();

    // Propagating Errors from reading a file in practice
    let username = read_username_from_file_standardly();

    // The following line results in a compilation error because the return Err type of
    // Result does not match the return main. `?` must be used in a function whose return type
    // matches the type `?` is used on.
    // Adding a return type on main allows the following to compile.
    let greeting_file = File::open("hello.txt")?;

    // Solutions to allow `?` to be used here
    // Change the return type of the function or use a match or one of the Result<T, E> methods to
    // handle the Result<T, E>

    // Using `?` on Option
    let mut last_char = last_char_of_first_line("");
    println!("Last char in string: {:?}", last_char);

    last_char = last_char_of_first_line("hello");
    println!("Last char in string: {:?}", last_char);

    // Included here as a return value associated with mains return type.
    Ok(())
}

// Propagate error back to caller
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Propagate error back to caller concisely
fn read_username_from_file_concisely() -> Result<String, io::Error> {
    //let mut username_file = File::open("hello.txt")?; // return error of type Result<String,
    // io::Error>
    let mut username = String::new();
    // username_file.read_to_string(&mut username)?;

    // Shorten the error propagation even more by chaining `?` operators.
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_standardly() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using the `?` on an Option type that checks if the str is empty. The empty string will return
// None at the first instance of next marked with `?` otherwise it will return the last char.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
