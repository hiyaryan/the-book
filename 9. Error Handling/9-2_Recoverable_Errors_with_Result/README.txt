Recoverable Errors with Result

Like Options, the Result enum is brought into scope by the prelude.

<T, E>: generic type parameters 
     T: Type of value to be returned on success
     E: Type of error to return on failure

Two variants of Result: Ok, and Err

enum Result<T, E> {
    Ok(T),
    Err(E),
}