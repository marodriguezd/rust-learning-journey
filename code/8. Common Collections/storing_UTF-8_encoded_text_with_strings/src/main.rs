fn main() {
    creating_a_new_string();
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