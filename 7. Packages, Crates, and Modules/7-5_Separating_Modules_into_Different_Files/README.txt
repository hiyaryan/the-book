Separating Modules into Different Files

How to move and use large pieces of code into and from separate files (same method for both crate roots src/lib.rs 
and src/main.rs).
1. Move code into a separate <directory_name>/<module_name>.rs file
2. Refer to the loaded file's code using a path to where it was declared (see ch 7.3)

Alternate File Path
Rust supports older style of file paths.

Ex. Module named front_of_house declared in the crate root
    src/front_of_house.rs  # new style
    src/front_of_house/mod.rs  # old style 

Ex. Module named hosting that is a submodule of front_of_house
    src/front_of_house/hosting.rs   # new style
    src/front_of_house/hosting/mod.rs  # old style

A mixture of both styles is not allowed in the same module.
The downside for using the old style is that there will be several files named mod.rs. 

Using the technique outlined in this chapter makes it so that modules of code that are moved to new files can be 
moved without any modification to the code.

The `mod` keyword declares modules which is looked for by Rust in the file of the same name as the module.


Summary
Packages can be split into multiple crates. Crates can be split into multiple modules.

One can refer to one module defined in another module.
    - This is done by specifying absolute or relative paths.
    - These paths can be brought into scope with `use`.
        - `use` makes it convenient to use a public module multiple times by providing a shorter path to the module.

Module code is private by default.
    - Use `pub` to make it accessible outside the scope of the module.