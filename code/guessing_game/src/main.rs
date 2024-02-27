// Import the functionality to generate random numbers from the `rand` crate.
use rand::Rng;
// Import the `Ordering` type from the standard library to compare numbers.
use std::cmp::Ordering;
// Import the input/output functionality from the standard library.
use std::io;

// Entry point of the program.
fn main() {
    // Print a message to the user.
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100.
    // `rand::thread_rng()` obtains a random number generator local to the current thread.
    // `gen_range(1..=100)` generates a number in the specified range, including both endpoints.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Display the secret number in the console (normally you wouldn't do this in a real game).
    println!("The secret number is: {secret_number}");

    // Start an infinite loop to allow multiple guesses.
    loop {
        println!("Please input your guess.");

        // Create a new string to store the user's guess.
        let mut guess = String::new();

        // `io::stdin().read_line(&mut guess)` reads the line from the standard input (`stdin`).
        // The `read_line` method appends the input to `guess` string and returns a `Result`.
        // `expect` handles any error that might occur while reading the line, terminating the program with the provided message if there's an error.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempt to convert the guess from a string to an unsigned 32-bit number (`u32`).
        // `trim` removes any whitespace at the beginning and end of the string.
        // `parse` converts the string into a number, returning a `Result`.
        // Use `match` to handle the `Result`: if `Ok`, extract the number; if `Err`, continue to the next iteration of the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the user's guess.
        println!("You guessed: {guess}");

        // Compare the guess to the secret number using `cmp`, which returns an `Ordering`.
        // Use `match` to decide what to do based on whether the guess is less than, greater than, or equal to the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If the guess is less, print "Too small!".
            Ordering::Greater => println!("Too big!"), // If the guess is greater, print "Too big!".
            Ordering::Equal => {
                println!("You win!"); // If the guess is correct, print "You win!" and exit the loop.
                break;
            }
        }
    }
}
