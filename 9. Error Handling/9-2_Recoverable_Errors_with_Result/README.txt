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

Shortcut for Panic on Error: unwrap and expect
Result<T, E> contains helper methods `unwrap` and `expect` that unwraps return value inside `Ok`. If the result is `Err`, unwrap will call the default `panic!` macro message, and except will allow a custom message.

Note that `except` is the choice for error catching in production. 


Propagating Errors
Propagating the error: Instead of calling the handling error within a function itself, return the error to the calling code so that it can decide what to do.
 
