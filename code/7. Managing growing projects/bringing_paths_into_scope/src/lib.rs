// Bringing Paths into Scope with the use Keyword
// This only works in the same scope, if we made another mod instead of a fn. It won't work
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Creating idiomatic use Paths

mod front_of_house_v2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/* Listing above is idiomatic because bringing the parent module into scope makes it clear the
 * function isn't locally defined, while minimizing path repetition.
 * Listing behind is unclear about where the function is defined.
 * For structs, enums, and other items, it's idiomatic to specify the full path...
 */

use crate::front_of_house_v2::hosting::add_to_waitlist;

pub fn eat_at_restaurant_v2() {
    add_to_waitlist();
}

/* The idiom of specifying the full path for structs, enums, etc., is a convention for clarity
 * and consistency.
 * Exception: When bringing in two items with the same name, use statements can't handle this.
 * Listing behind shows how to bring two Result types into scope from different parent modules.
 */

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    io::Result::Ok(())
}

// Providing New Names with the as Keyword

/* Another solution for bringing two types with the same name into the same scope is to use as to
 * create an alias after the path.
 */
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1_v2() -> Result {
    // --snip--
    Result::Ok(())
}

fn function2_v2() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}

// Re-exporting Names with pub use

/*
 * Combining pub and use re-exports a name, making it available for external code to use as if it
 * were defined in their scope.
 */

mod front_of_house_v3 {
    pub mod hosting_v2 {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house_v3::hosting_v2;

pub fn eat_at_restaurant_v3() {
    hosting_v2::add_to_waitlist();
}

/*
 * Before this change, external code had to use
 * restaurant::front_of_house::hosting::add_to_waitlist().
 * Now, external code can use restaurant::hosting::add_to_waitlist() due to re-exporting with
 * pub use.
 * Re-exporting helps align internal structure with external usage, making the library easier
 * to navigate and use.
 */