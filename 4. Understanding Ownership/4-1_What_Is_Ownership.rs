// What is Ownership?
// A set of rules that governs how Rust manages memory.

// Ownership (like garbage collection in Java) is Rust's way of managing memory.

// *No feature of ownership will slow down a program while running.*


// The Stack and the Heap
// Whether a value is on the stack or the heap changes how a program behaves and whi certain
// decisions are made.

// Stack and heap are parts of memory available at runtime but are structured in different ways.

// Stack
// Stores values in the order is gets them and removes values in the opposite order-LIFO "Last-in,
// First-out."
// Adding data is called pushing onto the stack.
// Removing data is called popping off the stack.
// All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time must be stored in the heap instead.

// Heap
// Data on the put on the heap must request a certain amount of space.
// Memory allocator finds an empty spot in the heap big enough, marks it being in use, and returns
// a pointer (the address of that location).
// Above is called allocating on the heap, or simply allocating.
// The pointer is then stored in the stack, which points to the data in the heap.

// General notes
// - Pushing on the stack is faster than allocating data on the heap because there is no allocator
// searching for a place to store data, its always at the top of the stack.
// - The allocator must find a big enough space to hold the data and then perform bookkeeping to
// prepare for the next allocation.
// - Accessing data on the heap is slower than accessing data on the stack because the pointer must
// be followed to get there-it takes time to reach data further away.
// - When code calls a function, the values passed into the function (including potential pointers)
// and the functions local variables get pushed onto the stack and popped off the stack when the
// function ends.

// *Ownership addresses keeping track of code using data on the heap, minimizing the amount of
// duplicate data on the heap, and cleaning up unused data on the heap to preserve space.*


// Ownership Rules 
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.


// Memory and Allocation

// Literals are hardcoded directly into the final executable because they are known at compile
// time. 
// This is fast and efficient because of it immutablity.

// The String type is unknown at compile time and whose size might change.
// To make String mutable its memory must be allocated on the heap.
// - Memory must be requested from the the memory allocator at runtime.
//       - `String::from` makes the request from the allocator.
// - This memory must be able to be returned to the allocator
//       - Normally a garbage collector keeps track and cleans up memory or the programmer keeps
//       track and cleans up the memory themself. 
//       - In Rust, the memory is automatically returned once the variable that owns it goes out of
//       scope.

// `drop`: a special function that is called when a variable goes out of scope
// - This is where the programmer can put the code to return the memory
// - It is called automatically at the closing curly bracket.


// Double free error 
// - Occurs when Rust tries to free memory twice
// - This is dealt with by invalidating the copied variable
//       - An invalidated variable can no longer be used
// Move
// - When a variable is invalidated, it is "moved" into the variable copying it.
// - Moved is used instead of "shallow copied" because of the invalidation
// - Since the copied variable wasa moved, the variable it was moved into will be responsible for
// dropping the memory from the heap.


// Deep Copies
// Rust will never automatically create deep copies of data by design due to the double free error
// Therefore, any automatic copying is inexpensive in terms of runtime performance.

fn main() {
    // Variable Scope
    // def. scope - the range within a program for which an item is valid.
    
    // The variable is valid from the point at which it's declared until the end of the current
    // scope.
    let s = "hello";

    // There are two important points:
    // - When 's' comes into scope becoming valid.
    // - When 's' goes out of scope becoming invalid.


    // The String Type
    // The String Type is stored on the heap and uses the principles of ownership to clean up
    // String literals like 's' above are immutable and are on the stack.
    // The String type manages heap data and can store data unknown at compile time.

    // Create a String from a string literal.
    // :: operator allows namespace of the particular 'from' function under String type
    let s = String::from("hello"); // 's' is now mutable as a String type and is in scope.

    // 's' is now mutable as a String type.
    // Strings can be mutated and literals cannot due to how they deal with memory.

    moving_variables_and_data();

    cloning_variables_and_data();

    copying_stack_data();

    ownership_and_functions();

    return_values_and_scope();
} // 's' goes out of scope and the variable is automatically returned


fn moving_variables_and_data() {
    // Multiple variables interacting with the same data.
    let x = 5; // bind 5 to x; push to stack
    let y = x; // make a copy; push to stack

    // String are made up of three parts
    // 1. pointer: points to the memory location
    // 2. length: how much memory in bytes is the current contents
    // 3. capacity: the total amount of memory that can be stored at that location.
    let s1 = String::from("hello"); // bind String allocator to s1; allocate to heap
    let s2 = s1; // make a copy of all three parts referencing the memory allocation; does not copy the data
    
    // The next line produces an error because s1 becomes invalidated once s2 makes a copy of it.
    // This is done to prevent the 'double free error'.
    // println!("{}, world!", s1);
}


fn cloning_variables_and_data() {
    // Deep copies of data can be made using the clone method.
    // Note that clone is an expensive method.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}


fn copying_stack_data() {
    let x = 5;
    let y = x;
    
    // Integers have a known size at compile time, and for this reason are stored on the stack.
    // Unlike with heap objects, there is no reason to make invalidate an integer after making a
    // copy of it
    // There is no difference between deep and shallow copying.
    // Variables stored on the stack are annotated with a Copy trait and are trivially copied
    // making them valid after assignment to another variable
    // For any heap objects annotated with Drop, the compiler will not allow it to be annotated
    // with Copy due to ownership rules.
    // Types that implement Copy:
    // - All integer types
    // - The Boolean type
    // - All floating types
    // - The character type
    // - Tuples that contain types that implement Copy
    println!("x = {}, y = {}", x, y);
}


fn ownership_and_functions() {
    // Passing a variable to a function will move or copy it just as assignment does.
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves ino the function and is no longer valid here
    // Trying to use s here throws a compile error
    // println!("{}", s);

    let x = 5; // x comes into scope

    makes_copy(x); // x moves into the function and is ok to use afterwards
} // x goes out of scope; since s's value was moved to a separate function nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string is dropped and memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens



// Assigning a value to another variables moves it every time.
// A variable that goes on the heap is annotated with Drop and is removed when out of scope unless
// it is Moved to another variable.
fn return_values_and_scope() {
    // Returning values also transfers ownership
    
    // moves ites return value into s1
    let s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into a function moving its return value into s3
    let s3 = takes_and_gives_back(s2);

    // Below produces an error since s2 is out of scope after being sent to takes_and_gives_back
    // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

    println!("s1 = {}, s3 = {}", s1, s3);


    let s4 = String::from("hello");

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);

} // s3 goes out of scope, s2 is moved, s1 goes out of scope.

// Moves return value into the function
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // comes into scope

    some_string // returned and moves out to calling function
}

// Takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // comes into scope
    a_string // returns and moves out to the calling function
}

// Rust allows multiple values to be returned using a tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// To prevent the necessity to return ownership everytime a variable on the heap is passed to a
// function, Rust has a feature called 'references'. See 4.2. References and Borrowing
