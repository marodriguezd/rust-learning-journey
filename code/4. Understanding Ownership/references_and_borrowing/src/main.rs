fn main() {
    referencing_a_value();

    mutable_references();

    multiple_mutable_references();

    combining_multiple_mutable_references();

    dangling_references();
}

fn referencing_a_value() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // doing in this way makes the "variable" immutable.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}  // Here, s goes out of scope. But because it does not have ownership of what
   // it refers to, it is not dropped.

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_mutable_references() {
    let mut s = String::from("hello");

    // This is going to make a crash due to multiple references changing the same value.
    /*
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    // The "correct" way to do so is not doing it at the same time nor scope.
    {
        let r1 = &mut s;
    }  // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;
}

fn combining_multiple_mutable_references() {
    let mut s = String::from("hello");

    // This is going to crash because, the reference to the value isn't a problem. But if you
    // reference it in some values and in one of them make it mutable, this can change the value
    // in all the references in the same scope and crash.
    // As an explanation this is due to the referencing order, because the mutability is in the
    // last one, this cannot affect the references before it in compilation time. But it's going to
    // make compiling errors.
    /*
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    let r3 = &mut s;  // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
     */

    // The correct way to do this would be
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;  // No problem
    println!("{}", r3);
}

fn dangling_references() {
    let reference_to_nothing = dangle();
}

// This function's return type contains a borrowed value, but there is no value
// for it to be borrowed from. So the compiler doesn't allow it.
/*
fn dangle() -> &String {  // dangle returns a reference to a String
    let s = String::from("hello");  // s is a new String

    &s  // we return a reference to the String, s
}  // Here, s goes out of scope, and is dropped. Its memory goes away
   // Danger!
 */