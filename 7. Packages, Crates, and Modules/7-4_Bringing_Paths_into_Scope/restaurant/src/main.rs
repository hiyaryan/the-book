// Idiomatic way to use structs, enums and other items is to follow the path to its declaration
use std::collections::HashMap;

// Exception to this rule is if the two items imported have the same name.
use std::fmt;
use std::io

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Only way to rell which Result belongs to which module is to specify the parent module explicilty.
fn func1() -> fmt::Result {
    // --snip--
}

fn func2() -> io::Result {
    // --snip
}