use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

fn main() {
    return_a_result_value_due_to_possible_fail(false);

    matching_on_different_errors(false);

    matching_on_different_errors_using_closures(false);

    shortcuts_for_panic_on_error(false);

    propagating_errors(false);

    a_shortcut_for_propagating_errors(false);

    where_the_interrogation_operator_can_be_used(true);
}

fn return_a_result_value_due_to_possible_fail(execute: bool) {
    if execute {
        let greeting_file_result = File::open("hello.txt");

        // Taking the easiest action handling the result with a match expression
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
    }
}

fn matching_on_different_errors(execute: bool) {
    if execute {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        };
    }
}

fn matching_on_different_errors_using_closures(execute: bool) {
    if execute {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
    }
}

fn shortcuts_for_panic_on_error(execute: bool) {
    if execute {
        let greeting_file_unwrap = File::open("hello.txt").unwrap();

        let greeting_file_expect = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }
}

fn propagating_errors(execute: bool) {
    fn read_username_from_file() -> Result<String, io::Error> {
        if super.execute {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        } else {
            // Mock returning a Result
            Ok(String::from(""))  //Mocked result: no operation performed"))
        }
    }
}

// As read_username_from_file in propagating_errors but shorter
fn a_shortcut_for_propagating_errors(execute: bool) {
    if execute {
        let mut username_file = File::open("hello.txt");
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username);

        // Making it more shorter
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username);

        // Even shorter using fs::read_to_string, anyway is preferable to use the longer way, because
        // this don't give the opportunity to explain the error handling.
        fs::read_to_string("hello.txt")
    }
}

fn where_the_interrogation_operator_can_be_used(execute: bool) {
    if execute {

    }
}