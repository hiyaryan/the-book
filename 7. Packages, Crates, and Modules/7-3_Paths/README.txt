Paths for Referring to an item in the Module Tree

Paths take two forms:
1. Absolute path-full path starting from a crate root; external crates begin with the crate name; the current crate starts with the literal `crate`.
2. Relative path-starts from the current module and uses `self`, `super`, or an identifier in the current module.

`::` separated the absolute/relative paths and the identifiers

Path to the public add_to_waitlist function
Absolute path: /front_of_house/hosting/add_to_wait
Relative path: front_of_house/hosting/add_to_waitlist

How to decide when to use the absolute path vs the relative path:
Depends on whether the item definition code is moved separately from or together with the code that uses the item.

Child modules wrap and hide their implementation details, but the child modules can see the context they're defined in.
Module system functions hide their implementation by default.

Expose inner parts of child modules with `pub`.

Best Practices for Packages with a Binary and a Library
1. Keep the default package names: src/lib.rs and src/main.rs
2. The module tree should be defined in src/lib.rs


Starting Relative Paths with `super`
Relative paths can begin in the parent module using super (similar to starting the filesystem path with `..`)
This allows items belonging in the parent module to be easily rearranged. 

Making Structs and Enums Public (use `pub`)
`pub` makes a struct public but its fields still remain private.
    Each field needs to be made public on a case-by-case basis.

`pub` makes an enum and all of its variants public.