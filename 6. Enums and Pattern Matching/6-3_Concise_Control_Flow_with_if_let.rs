// Concise Control Flow with `if let`
// `if let` combines `if` and 'into' a into a less verbose was to handle values that match
//     one pattern while ignoring the rest
// 
// `if let` is syntax sugar for `match` that runs code when the value matches one pattern
//     and then ifnores all others
//
// Use `if let` if program logic is too verbose to express using `match`. It prevents the
// necessity to exhaustively handle each enum variant.

fn main () {
    // The value of `Some` prints the `Some` variant by binding the value to the variable
    //     max in the pattern. We don't want anything to do with the `None` value and 
    //     have to add boilerplate code `_ => ()` to satisfy the `match expression`. 
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // To not have to include the boilderplate code for None in the situation where
    // nothing will happen if it is matched in the match expression use `if let`.
    // `if let` takes a pattern and expression separated by an equal sign. The pattern
    // is Some(max) and the `max` binds to the value inside the Some which allows it
    // to be used inside the body of the `if let` block. The code simply doesn't run 
    // if there's no match.

    // The tradeoff of using `if let` instead of `match` is exhaustive checking.
    // Generally speaking, if comes down to choosing conciseness over exhaustive 
    // checking.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}.", max);
    }

    // See 6-2 for Coin and UsState enums.
    // When `else` is included with `if let` the block of code within it is the same
    // as the `_` in the `match` block.
    // The following block counts all non-Quarter coins using and `match`.
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // Since only one pattern of the Coin type is matched here and all others are
    // ignored, `if let` and else could make above more readable.
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quaarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    // `if let` without an else ignores all non-target variant enums similar to `_ => ()`,
    // `if let` with `else` performs the same operation on  all non target variant enums
    //     similar to `_ => count += 1`. 
}

// Summary
// `enums` are used to create custom types that can be one of a set of enumerated values.
// The standard library contains the Option<T> type that helps prevent errors when using
//     the type system. 
// Data inside `enum` values can be extracted using `match` or `if let` depending on
//     how many variants need to be handled.