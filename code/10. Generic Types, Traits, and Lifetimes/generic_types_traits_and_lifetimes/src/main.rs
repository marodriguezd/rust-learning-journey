fn main() {
    removing_duplication_by_extracting_a_function();
}

fn removing_duplication_by_extracting_a_function() {
    // Generics help reduce code duplication by using placeholders for types.
    // First, let's extract duplicated code into a function to understand the process.
    // Later, we'll use generics to make this function more flexible.

    fn version1() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
    }

    fn version1_2() {  // Now 2 lists duplicating code
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
    }

    // Duplication is tedious and error-prone.
    // We'll create a function to handle any list of integers,
    // making the code clearer and abstracting the concept of finding the largest number.

    fn version2() {
        fn largest(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn main() {
            let number_list = vec![34, 50, 25, 100, 65];

            let result = largest(&number_list);
            println!("The largest number is {result}");

            let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

            let result = largest(&number_list);
            println!("The largest number is {result}");
        }
    }
}