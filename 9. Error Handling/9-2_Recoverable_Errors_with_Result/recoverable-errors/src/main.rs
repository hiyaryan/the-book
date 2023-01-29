use std::fs::File; // Return type of File::open is a Result<T, E>

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // If the result is Ok, the code will return the inner file value out of the Ok variant
    // If there is no file named hello.txt, the error will be seen out of the panic! macro.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}", error),
    };
}

