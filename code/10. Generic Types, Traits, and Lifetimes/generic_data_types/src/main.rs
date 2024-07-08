fn main() {
    in_function_definitions();

    in_struct_definition();

    in_enum_definition();

    in_method_definitions();

    performance_of_code_using_generics();
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

fn in_enum_definition() {
    // Similar to structs, enums can hold generic data types in their variants.
    // The Option<T> enum from Chapter 6 is an example:
    enum Option<T> {
        Some(T),
        None,
    }
    // Option<T> is generic over type T and has two variants: Some(T) and None.
    // It represents an optional value and can be used with any type.

    // Enums can use multiple generic types. For example, the Result enum from Chapter 9:
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    // Result<T, E> has two variants: Ok(T) for success and Err(E) for failure.
    // It is convenient for operations that might succeed (returning T) or fail (returning E).
}

fn in_method_definitions() {
    fn part1() {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        fn main() {
            let p = Point { x: 5, y: 10 };

            println!("p.x = {}", p.x());
        }

        // Here, Point<f32> has a method defined for f32 type only.
        // For that motive we needn't declare T in impl.
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        // Point<f32> will have the distance_from_origin method.
        // Other Point<T> types where T is not f32 will not have this method.
        // This method measures the distance from the origin (0.0, 0.0).
    }

    // This example demonstrates a situation where some generic parameters are declared with impl
    // and some are declared with the method definition. Here, the generic parameters X1 and Y1
    // are declared after impl because they go with the struct definition. The generic parameters
    // X2 and Y2 are declared after fn mixup because they’re only relevant to the method.
    fn part2() {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        fn main() {
            let p1 = Point { x: 5, y: 10.4 };
            let p2 = Point { x: "Hello", y: 'C' };

            let p3 = p1.mixup(p2);

            println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
        }

        main()  // To show
    }

    //part2()  // To show
}

// Exemplification of monomorphization
fn performance_of_code_using_generics() {
    // This to lines are using generics
    let integer = Some(5);
    let float = Some(5.0);

    // Internally the compiler works in this way and specify.
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_64::Some(5.0);
    }

    // The generic Option<T> is replaced with the specific definitions created by the compiler.
    // Because Rust compiles generic code into code that specifies the type in each instance,
    // we pay no runtime cost for using generics. When the code runs, it performs just as it would
    // if we had duplicated each definition by hand. The process of monomorphization makes
    // Rust’s generics extremely efficient at runtime.
}