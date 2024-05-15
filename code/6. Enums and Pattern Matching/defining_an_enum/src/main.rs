/* An Enum (enumeration) in Rust defines a type by enumerating its possible variants.
 * It's a way to group together a number of related values under a single data type. */
enum IpAddrKind {
    V4,
    V6,
}

/* This enum directly binds data to each variant, eliminating the need for an additional struct.
 * Each variant name, such as IpAddr::V4, acts as a constructor function that takes a String
 * and returns an enum instance, automatically provided by the enum definition.
 */
enum IpAddr {  // For association_between_enum_and_values()
    V4(String),
    V6(String),
}

/* Enums provide the flexibility to associate different types and amounts of data with each variant.
 * For instance, V4 addresses can be represented with four u8 values, while V6 addresses use a
 * single String. This varying data handling is not feasible with structs, highlighting the utility
 * of enums in such scenarios.
 */
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Any value of the enum have the same type, so we can define a function taking any IpAddrKind.
fn route(ip_kind: IpAddrKind) {}

fn main() {
    enum_values_and_implementations();

    enums_and_structs();

    association_between_enum_and_values();

    association_between_enum_and_values_advanced();

    another_example_with_enums_and_messages();

    option_examples_as_a_replacement_of_null();
}

fn enum_values_and_implementations() {
    // Instances of the values in the enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can call route with either varitan
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn enums_and_structs() {
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn association_between_enum_and_values() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

fn association_between_enum_and_values_advanced() {
    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));
}

fn another_example_with_enums_and_messages() {
    // Few variants in the same enum, some of which have zero or non use.
    enum Message {
        Quit,  // has no data associated.
        Move {x: i32, y: i32},  // has named fields, like a struct.
        Write(String),  // includes a single String.
        ChangeColor(i32, i32, i32),  // includes three i32 values.
    }

    /* Defining an enum with variants such as the ones in the block above is similar to defining
     * different kinds of struct definitions, except the enum doesn’t use the struct keyword and
     * all the variants are grouped together under the Message type. The following structs could
     * hold the same data that the preceding enum variants hold.
     */

    struct QuitMessage;  // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);  // tuple struct
    struct ChangeColorMessage(i32, i32, i32);  // tuple struct

    /* But if we used the different structs, each of which has its own type, we couldn’t as easily
     * define a function to take any of these kinds of messages as we could with the Message enum
     * defined in the block above, which is a single type.
     *
     * There is one more similarity between enums and structs: just as we’re able to define methods
     * on structs using impl, we’re also able to define methods on enums. Here’s a method named call
     * that we could define on our Message enum.
     */

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option_examples_as_a_replacement_of_null() {
    // The next exemplified enum is implicit in the standard library.
    /*
    enum Option<T> {
        None,
        Some(T),  // The <T> syntax is a generic type parameter.
    }
     */

    /**
     * The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a
     * different type. Rust can infer these types because we’ve specified a value inside the Some
     * variant. For absent_number, Rust requires us to annotate the overall Option type: the
     * compiler can’t infer the type that the corresponding Some variant will hold by looking
     * only at a None value. Here, we tell Rust that we mean for absent_number to be of type
     * Option<i32>.
     */

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}