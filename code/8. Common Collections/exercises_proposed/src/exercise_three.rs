use std::io::{self, Write};

pub fn main() {
    // Temporal example usage of the function
    let employee_name = get_user_input("Enter the employee's name: ");
    let department_name = get_user_input("Enter the department name: ");
    println!("Employee: {}, Department: {}", employee_name, department_name);
}

/// Function to get user input from the terminal
///
/// # Arguments
///
/// * 'prompt' - A string slice that holds the prompt message to be displayed to the user
///
/// # Returns
///
/// * A 'String' containing the user input
fn get_user_input(prompt: &str) -> String {
    // Print the prompt message to the user
    print!("{}", prompt);
    // Ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();

    let mut input = String::new();
    // Read the user's input from the terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim any leading and trailing whitespace from the input
    input.trim().to_string()
}