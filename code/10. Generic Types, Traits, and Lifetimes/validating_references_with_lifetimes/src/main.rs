fn main() {
    preventing_dangling_references_with_lifetimes();

    the_borrow_checker();

    generic_lifetimes_in_functions();

    lifetime_annotation_syntax();
}

fn preventing_dangling_references_with_lifetimes() {
    fn main() {  // This example doesn't compile due to Rust not allowing null values.
        let r;

        {
            let x = 5;
            r = &x;
        }  // r becomes a null value because the x ends up out of scope.

        println!("r: {r}");

        /*
         * The error message says that `x` “does not live long enough.” When the inner scope ends,
         * `x` will go out of scope, but `r` remains valid for the outer scope. Rust prevents this code
         * from running because `r` would then reference deallocated memory, which would be unsafe.
         * Rust uses a borrow checker to ensure all references are valid.
         */
    }
}

fn the_borrow_checker() {
    // The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
    // This is the same example as the function before but with annotations showing lifetimes.
    fn main() {
        let r;                // ---------+-- 'a
                                    //          |
        {                           //          |
            let x = 5;         // -+-- 'b  |
            r = &x;                 //  |       |
        }                           // -+       |
                                    //          |
        println!("r: {r}");         //          |
    }                               // ---------+

    // The way of fix this code, so it doesn’t have a dangling reference, and it compiles without any errors.

    fn working_main() {
        let x = 5;             // ----------+-- 'b
                                    //           |
        let r = &x;           // --+-- 'a  |
                                    //   |       |
        println!("r: {r}");         //   |       |
                                    // --+       |
    }                               // ----------+
    // Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can
    // reference x because Rust knows that the reference in r will always be valid while x is valid.
}

fn generic_lifetimes_in_functions() {
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }

    fn longest(x: &str, y: &str) -> &str {  // This doesn't compile.
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // Rust needs a lifetime parameter for the return type because it can't
    // tell if the returned reference is from `x` or `y`. The `if` block might return
    // a reference to `x`, and the `else` block might return a reference to `y`.
    // Without specifying lifetimes, Rust can't guarantee the returned reference will be valid.
}

fn lifetime_annotation_syntax() {

}