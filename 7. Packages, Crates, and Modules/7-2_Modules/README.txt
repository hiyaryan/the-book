Defining Modules to Control Scope and Privacy

Modules Cheat Sheet
1. Start from the crate root (src/lib.rs or src/main.rs)
2. Declaring modules (in the crate root): `mod <module-name>`
3. Declaring submodules (outside of the crate root): `mod <module-name>`
4. Paths to code in modules: code from modules within a crate can be access anywhere else in the same crate.
5. Private vs Public: Code inside a module is private by default. Make it public by prefixing the declaration with `pub`.
6. `use` keyword: creates shortcuts to items reducing repition to long paths

Binary Crate Illustration
backyard
    Cargo.lock
    Cargo.toml
    src
        garden
            vegetables.rs
        garden.rs
        main.rs

Crate Root File (main.rs) contains 
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden; // tells the compiler to include code it finds in src/garden.rs

#[derive(Debug)]
pub struct Asparagus {}

fn main(){
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

Grouping Related Code in Modules (ref. src/lib.rs)

e.g. Module tree and its crate roots

crate
    front_of_house
        hosting
            add_to_waitlist
            seat_at_table
        serving
            take_order
            serve_order
            take_payment
