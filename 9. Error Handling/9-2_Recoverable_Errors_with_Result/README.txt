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


A Shortcut for Propagating Errors: the `?` Operator
`?` operator provides a shortcut for the very common pattern of handling errors through propagation.

`?` placed after a `Result` value will propagate the Err back to the calling function if caught, otherwise the expression will return Ok.

`?` goes through the `from` function defined in the `From` trait in the standard library which converts one type to another. The error type received is converted to the type of the current function. 

Chain `?` operators together to make error propagation even more concise.


Where The ? Operator Can Be Used
`?` can only be used in functions whose return type is compatible with the value the `?` is used on.
The `?` operator is defined to perform an early return of a value out of the function.

Using `?` in main will result in a compilation error.
The return type of main is incompatible with the type the value `?` is used on.
...unless a return type is included on main.

`main` can return a Result<(), E>
Main can return any type that implements the `std::process::termination` trait that contains a function report that returns an `ExitCode`. See the standard library docs for more info on implementing the Termination trait. 

`?` can be used on the Option<T> type in a similar fashion to Result<T, E>. If the value is None, it will return early from the funciton, otherwise, Some is the resulting value.i

`?` can be used only on Option or Result, but not both in the same function. `?` will nit automatically convert one of these types to the other.
