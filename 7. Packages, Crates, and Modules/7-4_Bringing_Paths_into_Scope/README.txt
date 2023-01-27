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