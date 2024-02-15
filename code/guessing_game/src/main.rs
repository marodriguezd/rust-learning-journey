/*
 * To obtain user input and then print the result as output,
 * we need to bring the io input/output library into scope.
 * The io library comes from the standard library, known as std.
 */
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

    println!("You guessed: {guess}")
}