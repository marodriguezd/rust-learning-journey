use std::io;

fn main() {  // Data Types
    // Rust provides two floating-point types: f32 (32-bit) and f64 (64-bit, default). f64 is
    // preferred for its balance of speed and precision on modern CPUs. Both types are signed.
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32


    /*
     * Rust supports basic math operations: addition, subtraction, multiplication, division
     * (truncating towards zero for integers), and remainder. Each operation here evaluates to a
     * single value, assigned to a variable.
     */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    // In Rust, the Boolean type (`bool`) has two possible values: `true` and `false`, each taking
    // up one byte.
    let t = true;
    let f: bool = false; // with explicit type annotation


    // Rust's `char` type, denoted by single quotes, represents a Unicode Scalar Value and occupies
    // 4 bytes. It can encode a wide range of characters beyond ASCII, including emojis and
    // characters from various languages. This makes `char` capable of representing virtually any
    // character from the Unicode standard.
    let c = 'z';
    let z: char = 'Z';  // with explicit type anotation
    let heart_eyed_cat = '😻';


    // Compund Types:

    // Compound types can group multiple values into one type. Rust has two primitive compound
    // types: tuples and arrays.
    compound_types()
}

fn compound_types() {
    // Tuples in Rust group together values of different types into one compound type with a fixed
    // length. Values are comma-separated within parentheses. Optional type annotations can specify
    // the type of each element.
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // with explicit type annotation
    let tup = (500, 6.4, 1);

    // The `tup` variable is bound to the entire tuple, which is a single compound element. To
    // extract individual values, Rust allows pattern matching to destructure the tuple, as
    // demonstrated with `let (x, y, z) = tup;`, assigning each element to a separate variable.
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // This program demonstrates tuple destructuring and direct element access. `let (x, y, z) =
    // tup;` deconstructs the tuple into individual variables. Elements can also be accessed by
    // index using `tuple.index` notation, as shown with `x.0`, `x.1`, and `x.2`.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;


    // Arrays in Rust store multiple values of the same type with a fixed length. Values are written
    // as a comma-separated list inside square brackets.
    let array = [1, 2, 3, 4, 5];

    // Arrays in Rust are suitable for fixed-size collections allocated on the stack, offering
    // performance benefits over heap-allocated structures like vectors. While vectors are more
    // flexible in size, arrays are ideal when the number of elements is known and constant, such as
    // the 12 months of the year.
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // Array types are declared with square brackets, specifying the element type and array length
    // (`[i32; 5]`). Arrays can be initialized with specific values or a single value repeated for
    // each element (`[3; 5]` equals `[3, 3, 3, 3, 3]`).
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type with individual values
    let array = [3; 5]; // Implicit type with repeated value

    // Arrays in Rust are contiguous memory blocks with fixed size. Elements are accessed by
    // indexing, starting at 0. For example, `a[0]` retrieves the first element, `a[1]` the second,
    // and so on.
    let array = [1, 2, 3, 4, 5];

    let first = array[0];  // Accesses the first element, value 1
    let second = array[1]; // Accesses the second element, value 2

    // Invalid Array Element Access
    invalid_array_example()
}

fn invalid_array_example() {
    // Accessing an array element with an out-of-bounds index causes a runtime panic. This example
    // prompts for an index and retrieves the element at that position, demonstrating Rust's safety
    // check against invalid array indexing. Entering an index beyond the array size (e.g., 10 for
    // a 5-element array) results in a panic with an "index out of bounds" error.
    let array = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index]; // This line can panic if the index is out of bounds

    println!("The value of the element at index {index} is: {element}");
}