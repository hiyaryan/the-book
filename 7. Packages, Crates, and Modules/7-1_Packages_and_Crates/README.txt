Packages and Crates
def. crate - the smallest amount of code the Rust compiler considers at a time.

A single source code file is considered a crate.
Crates can contain modules that are defined in other modules that can get compiled with the crate.

Crate Forms
1. binary -  Programs that can be compiled to an executable and ran as a CLI program or a server.
             Each program must have a function called main that defines what happens when the executable runs.
2. library - Libraries don't compile to an executable and do not have a main function.
             Define functionality that can be shared with multiple projects.
                e.g. rand is a library
             The word crate generally refers to library when spoken.

def. crate root - a source file that the Rust compiler starts from and makes up the root module of a crate.
def. package - a bundle of one of more crates that provides a set of functionality

`Cargo.toml` file describes how to build those crates. 
`Cargo` is a package containing the binary crate for the CLI tool used to build code.
    Contains a library crate that the binary crate depends on.
    Other projects can use the same logic the Cargo CLI tool uses.

Packages 
    Can contain an unlimited number of binary crates, but at most 'one' library crate.
    Must contain at least one crate, whether library or binary crate.

```
$ cargo new my-project
    Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

The command `cargo new <name>` creates 
    a package indicated by the Cargo.toml file. 
    a src directory  containing main.rs


```Cargo.toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The Cargo.toml file does not mention src/main.rs. Cargo follows the convention that src/main.rs is the crate root
of the binary name with the same name as the package. Likewise, src/lib.rs indicates the package contains a library
crate with the same name as the package. Cargo passes the crate root files to rustc to build the library or binary.

A package containing main.rs and lib.rs in src contains two crates: a binary and library both with the same name
as the package. Multiple binary crates can be stored in src/bin directory with each file veing a separate binary crate.