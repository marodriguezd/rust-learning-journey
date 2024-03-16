mod fahrenheit_and_celsius;

fn main() {
    // if_expressions();

    // loop_expressions();

    fahrenheit_and_celsius::main();  // Exercise in another file
}

fn if_expressions() {
    // basic_if_statement();

    // else_if_statement();

    if_in_let_statement();
}

fn basic_if_statement() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn else_if_statement() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}")
}

fn loop_expressions() {
    // infinite_loop();

    // returning_values_from_loops();

    // nested_loops_and_breaks();

    // conditional_loops_with_while();

    for_looping();
}

fn infinite_loop() {
    loop {
        println!("again!");
    }
}

fn returning_values_from_loops() {
    let mut counter = 0;

    // Like in if_in_let_statement
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Implicit return
        }
    };

    println!("The result is {result}")
}

fn nested_loops_and_breaks() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LITOFF!!!");
}

fn for_looping() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    // In this way we risk a lot, tempting errors and possible crashes.
    /*while index < 5 {  // Using while like for
        println!("the value is: {}", a[index]);

        index += 1;
    }*/

    // This is a lot more highly secure way
    for element in a {
        println!("the value is: {element}");
    }

    println!(" ");

    // We can use a reverse loop too, this is an example
    // for number in (1..4).rev()  // In this way would be 4 3 2 1
    for number in a.iter().rev() {
        println!("{number}!");
    }

    println!("LITOFF!!!");
}