use std::collections::HashMap;

pub fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    calculations(numbers);
}

// Main function to perform calculations and display results
fn calculations(mut vec: Vec<i32>) {
    // Sort the vector in ascending order
    vec.sort();

    // Calculate the median
    let median = calculate_median(&vec);

    // Calculate the mode
    let mode = calculate_mode(&vec);

    // Display the results
    display_results(median, mode);
}

// Function to calculate the median
fn calculate_median(vec: &Vec<i32>) -> i32 {
    // Calculate the middle index
    let mid_index = vec.len() / 2;

    // Calculate the median depending on the length of the vector
    let median = match vec.len() % 2 {
        0 => {
            // If the length is even, return the average of the two middle values
            (vec[mid_index - 1] + vec[mid_index]) / 2
        },
        _ => {
            // If the length is odd, return the middle value
            vec[mid_index]
        }
    };

    median
}

// Function to calculate the mode
fn calculate_mode(vec: &Vec<i32>) -> i32 {
    let mut mode_map = HashMap::new();

    // Count the occurrences of each number in the vector
    for &number in vec {
        let count = mode_map.entry(number).or_insert(0);
        *count += 1;
    }

    // Variables to track mode and its count
    let mut mode = vec[0];
    let mut max_count = 0;

    // Iterate over the HashMap to find the number with the highest count
    for (number, count) in mode_map {
        // Update the mode if the current count is greater than the max_count
        if count > max_count {
            mode = number;
            max_count = count;
        }
    }

    mode
}

// Function to display the results
fn display_results(median: i32, mode: i32) {
    println!("The median is: {median}, and the mode is: {mode}");
}