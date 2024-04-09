fn main() {
    struct_instantiation();

    build_user(String::from("someone@example.com"), String::from("someusername123"));

    build_user_shorthanded(String::from("someone@example.com"), String::from("someusername123"));

    making_user2_inheritance_user1();

    making_user2_inheritance_user1_using_struct_update();

    using_tuple_structs();

    unit_like_structs_without_fields();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn struct_instantiation() {
    // Adding the mutability allow to change any parameter of it after the creation of the instance.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    // This is an example of changin one parameter of the mutable instance of the struct.
    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}

fn build_user_shorthanded(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn making_user2_inheritance_user1() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };
}

fn making_user2_inheritance_user1_using_struct_update() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
        // The ..user1 must come last to specify that any remaining fields should get their values
        // from the corresponding fields in user1.
    };

    // Struct update syntax behaves like data movement due to the `=` operator, affecting ownership.
    // For example, after `user1`'s data is used to create `user2`, `user1` might not be fully usable
    // if its fields' data were moved (not copied) into `user2`. Fields like `active` and
    // `sign_in_count` that implement the Copy trait aren't affected and remain usable in `user1`.

}

// Few tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn using_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// unit-like struct
struct AlwaysEqual;

fn unit_like_structs_without_fields() {
    let subject = AlwaysEqual;
}