use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self, Write};

pub fn main() {
    let mut company = HashMap::new();

    loop {
        // Get the action from the user
        let action = get_user_input("Choose an action: add, list, or quit: ");

        match action.as_str() {
            "add" => {
                // Get the employee's name and department from the user
                let employee = get_user_input("Enter the employee's name: ");
                let department = get_user_input("Enter the department name: ");
                add_employee(&mut company, &employee, &department);
            },
            "list" => {
                // Get the department name from the user
                // If 'all' is entered, list all employees in all departments
                let department = get_user_input("Enter the department name (or 'all' for all departments): ");
                if department == "all" {
                    list_all_employees(&company);
                } else {
                    list_employees_by_department(&company, &department);
                }
            },
            "quit" => break,
            _ => println!("Invalid action. Please try again."),
        }
    }
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
    // Flush to ensure the prompt appears before input

    let mut input = String::new();
    // Read the user's input from the terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim any leading and trailing whitespace from the input
    input.trim().to_string()
}

/// Function to list employees in a specific department
///
/// # Arguments
///
/// * 'company' - A reference to the HashMap representing the company
/// * 'department' - A string slice that holds the department's name
fn add_employee(company: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
    // Get the vector of employees for the department, or insert a new vector if it doesn't exist
    let employees = company.entry(department.to_string()).or_insert(Vec::new());
    // Add the employee to the department
    employees.push(employee.to_string());
    println!("Added {} to {}", employee, department);
}

/// Function to list employees in a specific department
///
/// # Arguments
///
///
/// * 'company' - A reference to the HashMap representing the company
/// * 'department' - A string slice that holds the department's name
fn list_employees_by_department(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => {
            // Clone and sort the list of employees
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("Employees in {}: {:?}", department, sorted_employees);
        },
        None => println!("No department named {}", department),
    }
}

/// Function to list all employees in the company, sorted by department
///
/// # Arguments
///
/// * 'company' - A reference to the HashMap representing the company
fn list_all_employees(company: &HashMap<String, Vec<String>>) {
    // Collect and sort the department names
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();

    // List employees for each department
    for department in departments {
        list_employees_by_department(company, department);
    }
}