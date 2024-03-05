fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  // Without 'mut' we can't do this.
    println!("The value of x is: {x}");

    // The `THREE_HOURS_IN_SECONDS` constant represents 3 hours in seconds, calculated as 60
    // seconds/minute * 60 minutes/hour * 3 hours. Rust conventions dictate using uppercase
    // with underscores for constants. This expression is evaluated at compile-time for clarity
    // and verification ease, rather than using a direct value like 10,800.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing in Rust allows declaring a new variable with the same name, hiding the previous
    // one. This example demonstrates shadowing by redeclaring `x` multiple times, each time with
    // a new value, and showing how it affects the variable's value within different scopes.
    // Initially, `x` is set to 5, then shadowed by a new `x` with a value of 6, and again within
    // an inner scope to 12, reverting back to 6 after the inner scope ends.
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing allows redeclaration of a variable with a new type, unlike `mut` which requires
    // the same type. This enables transforming a value and changing its type without needing
    // different variable names. For instance, transforming a string to its length, as shown here
    // with `spaces`. Using `mut` instead would result in a compile-time error due to type mismatch.
    let spaces = "   ";
    let spaces = spaces.len();
}
