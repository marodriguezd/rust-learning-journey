fn main() {
    in_function_definitions();

    in_struct_definition();
}

fn in_function_definitions() {
    // Using generics in function signatures makes our code flexible and avoids duplication.
    // The next function shows two functions that find the largest value in a slice of i32 and char.
    // We will later combine these into a single generic function.

    fn version1() {
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn main() {
            let number_list = vec![34, 50, 100, 65];

            let result = largest_i32(&number_list);
            println!("The largest number is {result}");

            let char_list = vec!['y', 'm', 'a', 'q'];

            let result = largest_char(&char_list);
            println!("The largest char is {result}");
        }
    }

    fn version2() {  // The PartialOrd is because we need to assure the item > largest compatibility.
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
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

            let char_list = vec!['y', 'm', 'a', 'q'];

            let result = largest(&char_list);
            println!("The largest char is {result}");
        }
    }
}

fn in_struct_definition() {
    fn generic_version() {  // Same generic to the struct
        struct Point<T> {
            x: T,
            y: T,
        }

        fn main() {
            let integer = Point { x: 5, y: 10 };
            let float = Point { x: 1.0, y: 4.0 };
        }
    }

    fn generics_version() {  // Possibly 2 generics to the struct
        struct Point<T, U> {
            x: T,
            y: U,
        }

        fn main() {
            let both_integer = Point { x: 5, y: 10 };
            let both_float = Point { x: 1.0, y: 4.0 };
            let integer_and_float = Point { x: 5, y: 4.0 };
        }
    }

    // All instances of Point shown are now allowed.
    // While you can use many generic type parameters, too many can make your code hard to read.
    // Needing many generic types may indicate that your code should be restructured into smaller pieces.
}