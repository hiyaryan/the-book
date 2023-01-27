Bringing Paths into Scope with the `use` Keyword
Use `use` to bring a function into scope to prevent having to specifying the absolute or relative paths everytime.

`use` is similar to creating a symbolic link in a filesystem. `use crate::parent_module_name::child_module_name` brings
its functions into scope. 

Ensure that `use` is used in its scope, use super::module_name or add the use statement into the separate module.


Creating Idiomatic use Paths
The idiomatic method helps makes it clear where a particular crate is coming from.

Idomatic method for bringing a function into scope
    `use crate::parent_module::child_module:`
It's now clear where the function is being called from
    `child_module::child_module_function`

Idiomatic method for bringing a struct, enum, or other item into scope (with no naming conflicts)
    `use std::collections::HashMap`
Using this is clear
    `let mut map = HashMap::new()`

Idiomatic method for bringing a struct, enum, or other item into scope (with naming conflicts)
    `use std::fmt::Result` // Not clear
    `use std::io::Result` // Not clear
Should be
    `use std::fmt`
    `use std::io`
Now it's clear which, `fmt::Result`, `io::Result`, Result is being used.


Providing New Names with the `as` Keyword
Also considered an idiomatic approach, use `as` to give an alias to two types of the same name into scope using `use`.
    `use std::fmt::Result`
    `use std::io::Result as IoResult` // It's clear this result is from the io module


Re-exporting Names with `pub use`
`use` brings items into private scope.
`pub use` brings items into public scope that enables other code to call it

Before `pub`, to call the code, the path was,
    `restaurant::front_of_house::hosting::add_to_waitlist()`
With `pub`,
    `restaurant::hosting::add_to_waitlist()`

`pub use` provides a means to structure code internally one way, and expose it a different way (convenient public API).
(see ch 14 for more information).


Using External Packages
List any external library to be used in the `Cargo.toml` file.

The `std` library is a crate that is shipped with rust so it does not have to be included in `Cargo.toml` as it is already
downloaded. Find crates at [url]:crates.io to add to the `Cargo.toml` file dependencies to tell Cargo to download them.

Filename: Cargo.toml
rand = "0.8.5"

Filename: main.rs
use rand::Rng; // imported
use std::collections::HashMap // absolute path 


Using Nested Paths to Clean Up Large use List
To save vertical space in code nest paths in a single list

Filename: src/main.rs
// This is better
use std::{cmp::Ordering, io};

// Than this
use std::cmp::Ordering;
use std::cmp::io;


Nested paths can be used at any level in a path

Filename: src/lib.rs
use std::io;
use std::io::Write;

// Write above on a single line
use std::io::{self, Write};


