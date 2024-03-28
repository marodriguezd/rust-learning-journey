fn main() {
    variable_standard_scope();

    the_capital_string_type();

    variables_and_data_interacting_with_move();

    ownerships_and_functions();

    return_values_and_scope();

    return_values_and_scope_multiple_returns_using_tuple();
}

fn variable_standard_scope() {
    {                           // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                           // this scope is now over, and s is no longer valid
}

fn the_capital_string_type() {
    // This example showcases the String type to demonstrate ownership rules in Rust. Unlike simple
    // data types stored on the stack, String is a complex type that manages data on the heap,
    // suitable for dynamically-sized text. Here, we use String::from to create a heap-allocated
    // String, highlighting how Rust manages memory and ownership, especially when the variable
    // goes out of scope.
    let s = String::from("hello");

    // This snippet demonstrates that Strings, unlike string literals, can be mutated. Here, we
    // declare a mutable String 's' and append a literal to it using push_str(). This mutability is
    // due to String being a complex type that manages heap-allocated memory, allowing for dynamic
    // modifications.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    allocation_of_capital_string_only_in_scopes();
}

fn allocation_of_capital_string_only_in_scopes() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
    // longer valid
}

fn variables_and_data_interacting_with_move() {
    let x = 5;
    let y = x;  // y is now a copy of the value of x because int is simple.

    // A complex type copy the pointer not the value.

    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2, not copied. s1 is no longer valid and cannot be used.
    // Due to Rust's ownership rules, when s2 goes out of scope, the memory is safely freed without
    // risk of double free errors. Attempting to use s1 after it has been moved to s2 will result
    // in a compile-time error, preventing potential memory safety issues.

    let s1 = String::from("hello");
    let s2 = s1.clone();  // s2 is a deep copy of s1, including heap data. Both are valid and independent.
    // Using clone makes an explicit deep copy of the String's heap data, avoiding the move
    // semantics of the previous example. Be mindful that cloning can be costly in terms of performance.

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownerships_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);                       // s's value moves into the function...
                                              // ... and so is no longer valid here

    let x = 5;                           // x comes into scope

    makes_copy(x);                            // x would move into the function,
                                              // but i32 is Copy, so it's okay to still
                                              // use x afterward
}  // Here, x goes out of scope, then s. But because s's value was moved, nothing
   // special happens.

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);
}  // Here, some_string goes out of scope and 'drop' is called. The backing
   // memory is freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let s1 = gives_ownership();                // gives_ownership moves its return
                                                      // value into s1

    let s2 = String::from("hello");         // s2 comes into scope

    let s3 = takes_and_gives_back(s2);         // s2 is moved into
                                                      // takes_and_gives_back, which also
                                                      // moves its return value into s3
}  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
   // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {                              // gives_ownership will move its
                                                              // return value into the function
                                                              // that calls it

    let some_string = String::from("yours");        // some_string comes into scope

    some_string                                               // some_string is returned and
                                                              // moves out to the calling
                                                              // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {          // a_string comes into
                                                               // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn return_values_and_scope_multiple_returns_using_tuple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len() returns the length of a String

    (s, length)  // To return the tuple with the 2 values.
}