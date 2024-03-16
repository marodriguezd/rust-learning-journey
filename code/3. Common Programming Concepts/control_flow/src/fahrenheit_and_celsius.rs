use std::io;

// Main entry point of the program
pub fn main() {
    let user_temperature = get_user_temperature();
    println!("You entered: {}", user_temperature);

    let user_type_of_measurement = get_type_of_measurement();
    match user_type_of_measurement {
        'C' | 'c' => println!("You entered: Celsius."),
        'F' | 'f' => println!("You entered: Fahrenheit."),
        _ => unreachable!(), // Indica que no se espera llegar a este punto
    }

    // Performs the temperature conversion based on the user's choice and displays the result.
    let transformed_temperature = measurement_transformation(user_temperature, user_type_of_measurement);
    println!("The result is: {transformed_temperature}º{}", user_type_of_measurement.to_uppercase());
}

// Reads user input from the command line.
fn get_user_input() -> String {
    let mut input = String::new();

    // 'io::stdin().read_line(&mut input)' reads user input until Enter is pressed.
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    // Implicit return: in Rust, the last expression of a function can be returned without using the 'return' keyword.
    input.trim().to_string()
}

/// Prompts the user to enter a temperature and repeats the prompt until a valid value is entered.
fn get_user_temperature() -> f32 {
    loop {
        println!("Introduce the temperature:");

        let temperature = get_user_input();

        match temperature.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("The input isn't a valid number, please try again.")
        }
    }
}

/// Asks the user to choose between Celsius and Fahrenheit, repeating until a valid input is received.
fn get_type_of_measurement() -> char {
    loop {
        println!("Which kind of measurement do you prefer? To Celsius (C) or Fahrenheit (F):");

        // Assumes the user will only input one character and uses it directly.
        let measurement_type = get_user_input().chars().next().unwrap();

        match measurement_type.to_lowercase().next().unwrap() {
            'c' | 'f' => return measurement_type,
            _ => println!("Please enter 'C' for Celsius of 'F' for Fahrenheit."),
        }
    }
}

// Converts the user's temperature from Celsius to Fahrenheit or vice versa, based on the user's choice.
fn measurement_transformation(temperature: f32, measurement: char) -> f32 {
    match measurement {
        'C' | 'c' => {
            return (temperature - 32.0) * 5.0 / 9.0  // Conversion to Celsius.
        },
        'F' | 'f' => {
            return (temperature * 9.0 / 5.0) + 32.0  // Conversion to Fahrenheit
        },
        _ => {
            eprintln!("Invalid measurement type encountered: {measurement}");
            0.0  // Returning a default value or consider another error handling strategy.
        }
    }
}