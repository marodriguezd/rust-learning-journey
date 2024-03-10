fn main() {
    // another_function(5);

    // print_labeled_measurement(5, 'h');

    // example_of_expressions();

    example_of_function_with_return_values();
}

fn example_of_function_with_return_values() {
    let x = five();

    println!("The value of x is: {x}")

    second_example_of_function_with_return_values()
}

fn five() -> i32 {
    5  // Implicit return
}

fn second_example_of_function_with_return_values {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn example_of_expressions() {
    let y = {
        let x = 3;
        x + 1  // Like an implicit return
    };

    println!("The value of y is: {y}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}