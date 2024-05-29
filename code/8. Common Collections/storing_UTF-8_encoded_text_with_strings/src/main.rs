fn main() {
    creating_a_new_string();

    updating_a_string();
}

fn creating_a_new_string() {
    // String operations are similar to Vec<T> operations.
    // String is a wrapper around a vector of bytes.
    let mut s = String::new();

    // Initializing a string with data using the to_string method.
    let data = "initial contents";
    let s = data.to_string();

    // The method also works directly on a literal.
    let s = "initial contents".to_string();

    // Creates a string containing initial contents.
    let s = String::from("initial contents");

    /*
     * Both "initial contents".to_string() and String::from("initial
     * contents") create a String from a string literal.
     * - "initial contents".to_string() calls the to_string method
     * on a string slice, which is available because the Display
     * trait is implemented for string slices.
     * - String::from("initial contents") is a direct way to create
     * a String from a string literal.
     * Both methods are equivalent and the choice between them can
     * be a matter of style and readability.
     *
     * Strings are used for many things, offering multiple APIs for
     * different needs.
     * In this case, String::from and to_string do the same thing,
     * so which you choose is a matter of style and readability.
     */

    // Strings are UTF-8 encoded, so we can include any properly
    // encoded data in them. All of these are valid String values:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn updating_a_string() {
    // Appending to a String with push_str and push
    {
        // We can grow a String by using the push_str
        // method to append a string slice.
        let mut s = String::from("foo");
        s.push_str("bar");

        // After these two lines, s will contain "foobar".
        // The push_str method takes a string slice because we
        // don’t necessarily want to take ownership of the parameter.
    }

    {  // How to maintain "s2" after appending its contents
        let mut s1 = String::from("foo");
        let s2 = "bar";  // Using a variable to allocate this can retain it.
        s1.push_str(s2);
        println!("s2 is {s2}");

        // If the push_str method took ownership of s2, we wouldn’t
        // be able to print its value on the last line. However, this
        // code works as we’d expect!
    }

    {  // The push method takes a single character and adds it to the
       // String.
        let mut s = String::from("lo");
        s.push('l');
    }

    // Concatenation with the + Operator or the format! Macro

    {  // Combining two existing strings with "+" operator.
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;  // s1 has given up it ownership.

        // `s2` has an &, meaning we're adding a reference of the
        // second string to the first string.
        // This is because of the `s` parameter in the `add` function:
        // we can only add a `&str` to a `String`; we can't add two
        // `String` values together.

        // If we need to concatenate multiple strings, the behavior
        // of the + operator gets unwieldy:
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;

        // It’s difficult to see what’s going on.
        // We can instead use the format! macro:
        let s1 = String::from("tic");
        // s2 and s3 still alive

        let s = format!("{s1}-{s2}-{s3}");

        // The `format!` macro creates a `String` like `println!`,
        // but returns it instead of printing.
        // It’s easier to read and doesn’t take ownership of its
        // parameters.
    }
}