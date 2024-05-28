fn main() {
    creating_new_vector();

    updating_a_vector();

    reading_elements_of_vectors();

    iterating_over_vector_values();

    using_enums_to_store_multiple_types();
}

fn creating_new_vector() {
    // An empty vector
    let v: Vec<i32> = Vec::new();

    // Use the vec! macro to create a new Vec<T> auto-inferring the type.
    // And has the new() implicit.
    let v = vec![1, 2, 3];

}

fn updating_a_vector() {
    // To add elements into a vector we use push
    let mut v = Vec::new();  // We need a mutable one, obvious.

    // After the first push auto-infer the type.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn reading_elements_of_vectors() {
    // Reference a value in a vector using indexing or the get method.
    // Both return a reference, but get returns an Option for safe access.

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iterating_over_vector_values() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Use the * dereference operator to modify the value referred by
    // a mutable reference.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn using_enums_to_store_multiple_types() {
    // This is an example of a SpreadsheetCell enum that can store
    // multiple types (i32, f64, String).
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // This is an example of how to use the SpreadsheetCell enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Rust requires knowing vector types at compile time for memory
    // allocation.
    // Enums ensure type safety and handling all cases with match
    // expressions.

    // An added example of how to iterate over the enum vector
    for cell in &row {  // You can have infinite cells(enums)
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("String: {}", s),
        }
    }
}