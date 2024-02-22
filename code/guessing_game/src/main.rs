/*
 * To obtain user input and then print the result as output,
 * we need to bring the io input/output library into scope.
 * The io library comes from the standard library, known as std.
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
 * The main function is the entry point into the program.
 *
 * The fn syntax declares a new function;
 * the parentheses, (), indicate there are no parameters;
 * and the curly bracket, {, starts the body of the function.
 */
fn main() {
    // println! is a macro that prints a string to the screen.
    println!("Guess the number!");

    /*
     * We use `rand::thread_rng()` to get a random number generator local to the current thread,
     * seeded by the operating system. The `gen_range` method from the `Rng` trait, which is brought
     * into scope with `use rand::Rng;`, generates a random number within a specified range.
     * The range syntax `start..=end` is inclusive, so `1..=100` specifies a range between 1 and 100.
     */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    /*
     * We’ll create a variable to store the user input.
     *
     * We use the let statement to create the variable, In Rust, variables are immutable by default.
     * So they are like constants per se. And to make a mutable variable is with 'mut' after 'let'.
     */
    let mut guess = String::new();
    // String::new is a function that returns a new instance of a String
    // The :: syntax in the ::new line indicates that's an associated function of the String type.

    // The stdin function from the io module, will allow us to handle user input.
    io::stdin()
        /*
         * Calls the read_line method on the standard input handle to get input from the user.
         * We’re also passing &mut guess as the argument to read_line to tell it what string to
         * store the user input in.
         *
         * The & indicates that this argument is a reference, which gives you a way to let multiple
         * parts of your code access one piece of data without needing to copy that data into
         * memory multiple times.
         *
         * Like variables, references are immutable by default. Hence, you need to write
         * &mut guess rather than &guess to make it mutable.
         */
        .read_line(&mut guess)
        /*
         * The 'read_line' method reads user input into a string and returns a 'Result' enum
         * to handle potential errors. The 'Result' enum has two variants: 'Ok' and 'Err'.
         * 'Ok' indicates success and contains the resulting value, while 'Err' signifies failure
         * and contains error information. This mechanism is used for error handling in Rust.
         */
        .expect("Failed to read line");

    /*
     * Rust's type system and type inference lead to type mismatch errors when comparing strings and numbers.
     * The program uses shadowing to reuse the variable name `guess` for type conversion from `String` to a numeric type (e.g., `u32`).
     * This process involves trimming whitespace and parsing the string into a number, handling potential parse errors with `Result`.
     * The `trim` method removes whitespace and newline characters, making the string ready for numeric comparison.
     * Parsing converts the string to a `u32`, allowing comparison with the secret number, also inferred as `u32`.
     * Shadowing and parsing demonstrate Rust's approach to variable reuse and type safety.
     */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    /*
     * The `println!` macro uses `{}` as placeholders to print values. Placeholders can hold
     * variable values or the results of expressions. For variables, include their names inside
     * `{}`. For expressions, use `{}` without names, and list the expressions after the format
     * string, separated by commas.
     */
    println!("You guessed: {guess}");

    /*
     * We introduce std::cmp::Ordering to handle comparisons, which can be Less, Greater, or Equal.
     * Using the cmp method, we compare two values (e.g., guess and secret_number) and receive an Ordering enum variant.
     * A match expression then processes this variant to execute corresponding logic based on the comparison result.
     * Match arms, each consisting of a pattern and associated code, allow for handling various outcomes.
     * For instance, comparing a guess of 50 with a secret number of 38 returns Ordering::Greater, triggering the "Too big!" response.
     * This demonstrates Rust's pattern matching and control flow capabilities in handling comparisons and outcomes.
     */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
