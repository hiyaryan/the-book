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


Matching on Different Erros
Add inner match expressions in the error variant block of a outer match block to catch different kinds of failures.


Alternatives to Using `match` with Result<T, E>
Result<T, E> contains closures used to catch errors (see ch 13 for more on closures). These closures wrap error catching neatly without the necessity to nest match cases.
