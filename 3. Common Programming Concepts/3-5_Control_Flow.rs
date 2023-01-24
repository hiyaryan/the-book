// Control Flow
// The ability to code once or repeatedly depending on a truth condition.
// Most common constructs that control the flow of Rust code are 'if' expressions and loops.

// 'if' expressions
// - Branches code based on condition
// - The condition must return a bool type.
// - Non-bool types are not converted to boolean (i.e. 0 ≠ true, 1 ≠ false).

fn main() {
    let number = 3;
    
    // Checks the condition then runs the associated arm.
    // def. arm - the block of code associated with the if conditions
    if number < 5 {
        println!("condition was true");
    
    // Executes if if-condition is not satisfied.
    // If no else block is provided the remaining code in the method is executed.
    } else {
        println!("condition was false");
    }

    // Conditions must be explicit.
    // The commented out code on the next line  doesn't work because Rust doesn't convert non-bool types to bool.
    // if number { 
    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with 'else if'
    let number = 6;

    // Rust only executes the block for the first true condition.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } 

    // Using 'if' in a 'let' Statement
    // Since if is an expression, if can be used on the right side of a let statement and
    // assign its value to the variable.
    let condition = true;

    // Recall: blocks of code evaluate to the last expression in them (hence no ';')
    // Both values must be of the same type else an error will be thrown.
    let number = if condition { 5 } else { 6 };

    // Throws a type mismatch error.
    // Rust must know at compile time definitively what type of number the variable is.
    // Knowing the type of number allows the compiler to verify that it can be used everywhere it
    // is needed.
    // let number = if condition { 5 } else { "six" };
    println!("The value of the number is: {number}");


    // Repetition with Loops
    // Three kinds of loops: 'loop', 'while', 'for'

    // Repeating Code with 'loop'
    // 'loop' repeats code indefinitely until explicitly told to stop.
    // 'break' breaks out of a loop 
    // 'continue' skips over any remaining code in the loop and starts the next iteration.
    let mut i = 0;
    loop {
        println!("again!");

        if i == 5 {
            break;
        } else {
            i = i + 1;
        }
    }

    // Returning Values from Loops
    
    // 'loop' can be used on operations that might fail.
    //     e.g checking whether a thread has completed its job.
    //         Once the value has been obtained `break` from the expression
    //         The value will then be returned out of the loop.

    let mut counter = 0;
    
    // Notice how result is assigned to the value returned on break.
    let result = loop {
       counter += 1;

       if counter == 10 {
           break counter * 2;
       }
    };

    println!("The result is {result}");


    // Loop Labels to Disambiguate Between Multiple Loops

    // `break` and `continue` can be applied to the loop that uses them.
    // If this loop is nested, `break` and `continue` can be applied to the outer most loop within
    // the inner most loop using a label on the outer most loop.
    // Labels are applied to loops starting with a single quote followed by the name then a colon:
    let mut count = 0;
    // Label a loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
    

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Break from the inner loop then continue on the outer loop
                break;
            }
            
            if count == 2 {
                // Break from the outer loop and escape the nested loops.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    // Conditional Loops with `while`
    // While the condition is true the while loop runs, otherwise the program will break.
    // While loop is a combination of loop, if, else, and break.
    // While loops should not be used to loop through a collection-this is error prone and slow
    // because the compiler adds runtime code to perform the conditonal check every iteration
    // through the loop.
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!");
   

    // Looping Through a Collection with 'for'
    // 'for' loops increase the safety of the code for iterating through a collection-changing the
    // number of values in a collection does not require changing any other part of the code.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }

    // Use 'Range' from the standard library that generates numbers in sequence from a starting
    // point to an ending point.
    // Below loops in reverse from 4-exclusive to 1-inclusive.
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!")
}

