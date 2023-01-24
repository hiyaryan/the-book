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

// Using Tuple Structs without Named fields to Create Different Types
// Tuple structs: structs that look like tuples
// Fields don't have names so the struct name provides all of the information
// Useful for giving a whole tuple a name and make the tuple a different type from other tuples
//     and when naming each field would be verbose and redundant
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
// Structs without any field behave similarly to (), the unit type mentioned in "The Tuple Type"
// Useful for implementing a trait on some type without data to store.
// Later the behavior can be implemented-e.g. having an instance that is always equal (AlwaysEqual) 
//     to any other type.
// See: Chapter 10 for more information on how to define traits and implement them on any type including
//     unit-like structs.
struct AlwaysEqual;

// Ownership of Struct Data
// In the User struct above, the owned String type rather tham &str string slice type is used
//     deliberately because we want each instance of the struct to own all of the data and 
//     for that data to be valid for as long as the entire struct is valid
// Structs can also store references to data owned by something else-this requires 'lifetimes'
//     See: Chapter 10 for more information on lifetimes which ensure data referenced by structs
//     are valid for as long as the struct is alive
// Trying to store a reference in a struct without specifying lifetimes won't work
// struct UserNoLifetime {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

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
    user2.email = String::from("anotheremail@example.com");

    // Creating Instances from Other Instances with Struct Update Syntax
    // Create a new instance of a struct including most of the values from another instance.
    let user3 = User {
        active: user2.active,
        username:user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user2.sign_in_count,
    };

    // Do the same thing above with Struct Update Syntax
    // README: recall that data transfers ownership. All other user2 attributes have been given to user3 in the
    //     previous assignment. Only user2.email is still owned by user2. user3 then had to take ownership
    //     of whatever is owned by user1 minus whatever it took from user2, so user1 now only owns an email.
    let user3 = User {
        email: user2.email,
        ..user1
    };


    // See: Using Tuple Structs without Named Fields to Create Different Types
    // The following lines are considered to be different types because they are instances of different
    // structs.  
    // Each defined struct is its own type even if the fields in the struct are the same types.
    // Recall: With tuples, you can destructure them into individual pieces and a '.' can bew used
    // followed by the index to access an individual value.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // Unit-Like Structs without any Fields
    let subject = AlwaysEqual;

    // This fails because you can't store a reference in a struct without lifetimes
    // let user4 = UserNoLifetime {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1
    // }
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
