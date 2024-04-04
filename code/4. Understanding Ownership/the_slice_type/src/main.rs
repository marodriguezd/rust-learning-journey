/** EXAMPLE EXERCISE:
 * Here’s a small programming problem: write a function that takes a string of words separated by
 * spaces and returns the first word it finds in that string. If the function doesn’t find a space
 * in the string, the whole string must be one word, so the entire string should be returned.
 */

fn main() {
    // This have few syncing issues.
    without_using_slices();

    using_slices();

    string_slices_as_parameters();

    slices_in_other_types();
}

fn without_using_slices() {
    let mut s = String::from("hello world");

    let word = first_word_without_slices(&s);  // word will get the value 5

    s.clear();  // this empties the String, making it equal to ""

    // word strill has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word si now totally invalid!
}

fn first_word_without_slices(s: &String) -> usize {
    let bytes = s.as_bytes();  // Convert String into an array of bytes.

    // Why use enumerate: return a tuple. The first element of the tuple returned from enumerate is
    // the index, and the second element is a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {  // Iteration over the array of bytes.
        if item == b' ' {  // b' ' is byte literal syntax.
            return i;  // If we find a space we'll return the position.
        }
    }

    s.len()  // If we don't find a space we'll return the length of the String.
    // But because it’s a separate value from the String, there’s no guarantee that it will still be
    // valid in the future.
}

fn using_slices() {
    // A slice is a reference to a part of a String
    let s = String::from("hello world");

    // We create slices using a range within brackets by specifying [starting_index..ending_index]
    // starting_index is the first position in the slice and
    // ending_index is one more than the last position in the slice.
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word_for_using_slices(&s);

    println!("the first word is: {}", word);
}

fn first_word_for_using_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // Start of the String .. Index of the space
            // We get back a single value that is tied to the underlying data. The value is made up
            // of a reference to the starting point of the slice and the number of elements in the
            // slice.
        }
    }

    &s[..]
}

fn string_slices_as_parameters() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String's`, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `firs_word` also works on references to `String's`, which are equivalent
    // to whole slices of `String's`
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // Start of the String .. Index of the space
            // We get back a single value that is tied to the underlying data. The value is made up
            // of a reference to the starting point of the slice and the number of elements in the
            // slice.
        }
    }

    &s[..]
}

fn slices_in_other_types() {
    let a = [1, 2, 3, 4, 5];

    // This slice has the type &[i32]
    let slice = &a[1..3];
    // It works the same way as string slices do, by storing a reference to the first element and a
    // length. You’ll use this kind of slice for all sorts of other collections.

    assert_eq!(slice, &[2, 3]);
}