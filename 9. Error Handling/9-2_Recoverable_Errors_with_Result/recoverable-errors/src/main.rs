use std::fs::File; // Return type of File::open is a Result<T, E>
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
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
}

// Handles the error
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
