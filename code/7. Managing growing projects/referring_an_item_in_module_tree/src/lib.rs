mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

/* Calling add_to_waitlist in eat_at_restaurant:
 * Absolute path: starts with crate and lists all modules to the function.
 * Relative path: starts with front_of_house, same level as eat_at_restaurant.
 * Choose absolute paths for independent module moves, relative for joint moves.
 */

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/* Error: module hosting is private. Rust defaults to items being private.
 * Parent modules can't access private items in child modules, but children can access ancestor
 * modules. This ensures implementation details are hidden. Use pub to make items public if needed.
 */

// Now we are going to update and recreate this with pub and marking it using _v2

mod front_of_house_v2 {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

/* Adding pub to mod hosting makes the module public, but its contents remain private.
 * pub on a module allows ancestor modules to refer to it, not access its inner code.
 * To access inner code, make items within the module public as well.
 */

pub fn eat_at_restaurant_v2() {
    // Absolute path
    crate::front_of_house_v2::hosting::add_to_waitlist();

    // Relative path
    front_of_house_v2::hosting::add_to_waitlist();
}

// Now we are going to update and recreate this with pub and marking it using _v3

mod front_of_house_v3 {
    // Let’s also make add_to... function public by adding the pub keyword before its definition.
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant_v3() {
    // Absolute path
    crate::front_of_house_v3::hosting::add_to_waitlist();

    // Relative path
    crate::front_of_house_v3::hosting::add_to_waitlist();
}

/* Now the code will compile! Adding pub to hosting allows access via absolute and relative paths.
 * Absolute path: starts with crate, front_of_house and eat_at_restaurant are siblings, then access
 * hosting::add_to_waitlist. Relative path: starts from front_of_house, then access
 * hosting::add_to_waitlist. Both work as hosting and add_to_waitlist are marked pub.
 */

// The next part diverges a bit from before

/* Use super to construct relative paths starting from the parent module, similar to .. in
 * filesystems. This helps reference items in the parent module, aiding in rearranging the module
 * tree. Example: back_of_house::fix_incorrect_order calls deliver_order in the parent module
 * using super.
 */

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    /* fix_incorrect_order uses super to access deliver_order in the parent module (crate root).
     * This keeps the relationship between back_of_house and deliver_order intact, easing future
     * reorganization.
     */


    fn cook_order() {}
}

// The next part diverges a "lot" more from before

/* Use pub to make structs and enums public. Adding pub before a struct makes it public, but fields
 * remain private by default.
 * Each field can be made public individually. Example: back_of_house::Breakfast struct has a
 * public toast field and a private seasonal_fruit field.
 */

mod back_of_house_v2 {  // We're going to name it .._v2
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_v4() {  // We're going to name it .._v4
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house_v2::Breakfast::summer("Rye");
    // Change out mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

/* The public toast field in back_of_house::Breakfast allows read/write access in
 * eat_at_restaurant using dot notation.
 * The private seasonal_fruit field remains inaccessible. A public associated function (summer)
 * is needed to construct Breakfast instances due to the private field.
 * Making an enum public makes all its variants public with pub before the enum keyword.
 */

mod back_of_house_v3 {  // We're going to name it .._v3
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_v5() {  // We're going to name it .._v5
    let order1 = back_of_house_v3::Appetizer::Soup;
    let order2 = back_of_house_v3::Appetizer::Salad;
}

/* The public Appetizer enum allows using Soup and Salad variants in eat_at_restaurant.
 * Enum variants are public by default to avoid repetitive pub annotations, while struct fields
 * are private by default unless specified otherwise.
 */
