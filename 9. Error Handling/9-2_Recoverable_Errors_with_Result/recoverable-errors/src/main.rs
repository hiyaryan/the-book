use std::fs::File; // Return type of File::open is a Result<T, E>
use std::io::ErrorKind;

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
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}
