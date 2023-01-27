Bringing Paths into Scope with the `use` Keyword
Use `use` to bring a function into scope to prevent having to specifying the absolute or relative paths everytime.

`use` is similar to creating a symbolic link in a filesystem. `use crate::parent_module_name::child_module_name` brings
its functions into scope. 

Ensure that `use` is used in its scope, use super::module_name or add the use statement into the separate module.