// The `match` Control Flow Construct
// `match` compares a value against a series of patterns and then executes code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards, and many others (see ch 18)

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // UsState type added to indicate only Quarters from 1999-2008 may include state designs.
}

// Patterns that Bind to Values
// Match arms can bind to parts of values that match the pattern allowing values from enum variants to be extracted.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// The match expression can return any value (unlike an if that can only return a boolean)
// These expressions are composed of arms that are used to match the returned value of the match expression.
// The code in each arm is an expression that returns a resulting value for the entire match expression.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // If there is more than a single expression in an arm {} must be used.
        Coin::Penny => { 
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // variable state is added to the match expression that matches values of variant Coin::Quarter
                                  // when a Coin::Quarter matches, the state variable will bind to the value of that quarter's state.
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


fn main() {
    // Example using a matching a coin to Quarter with an Alaska design.
    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The coin has a value of {}.", quarter);
}