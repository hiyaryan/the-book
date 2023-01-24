// Defining and Instantiating Structs

// Structs are similar to tuples
//      - Both hold multiple related values
//      - Pieces of a struct can be different types

// Structs
//      - Each piece of data must be named
//      - More flexible than tuples
//      - Order of values is not necessary to specify or access values of an instance.

// Define a Struct with keyword `struct`
// Struct name should describe significance of the pieces of data being grouped.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    // Create an instance of the Struct using {key: value, ...} pairs
    // Specify concrete values for each of the field
    // Fields do not have to be specified in the same order in which they are declared in the struct
    // Struct definition is a general template for the type, and 
    // Instances fill in the the template with particular data to create values of that type
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Use dot-notation to get a value from the struct.
    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername1232"),
        active: true,
        sign_in_count: 1,
    };

    // Ensure that the entire instance of the struct is mutable if an attribute is to be updated.
    // A single value cannot only be marked as mutable, the entire struct must be.
    user2.email = String::from("anotheremail@example.com")
}


// A struct can be implicitly returned through a function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


// Creating a function that builds a struct like build_user using shorthand notation
// The 'field init shorthand syntax' can be used to rewrite the function above behaving in the exact same way
fn build_user_shorthand(email: String, username: String) -> User {
    // If the function parameter and field value has the same name, if only needs to be typed once.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// Creating Instances from Other Instances with Struct Update Syntax