// It's used around the code.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // New addition for the lasting blocks of code
    /* Defines the `can_hold` method within the `impl Rectangle` block.
     * This method takes an immutable borrow of another `Rectangle` as a parameter, indicated by
     * usage such as `rect1.can_hold(&rect2)`. It only needs to read the properties of the borrowed
     * `Rectangle`, hence an immutable borrow is sufficient. The method returns a Boolean value,
     * true if `self` can completely contain the other `Rectangle` based on comparing their width
     * and height.
     */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /* Creates a square `Rectangle` where the width and height are equal.
     * This is an associated function, not requiring an instance of `Rectangle`.
     * Usage: `Rectangle::square(size)` returns a new `Rectangle` instance with specified dimensions.
     */
    // Just like String::from to make the things easier to create.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    defining_methods();

    example_of_width_method_inside_a_rectangle();

    methods_with_more_parameters();

    example_for_making_a_square_using_associated_function();
}

// Let’s change the area function that has a Rectangle instance as a parameter and instead make an
// area method defined on the Rectangle struct. Referring to the previous lesson.
fn defining_methods() {
    // struct Rectangle was here
    // fn area was here

    main();

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}

/* The main reason for using methods instead of functions, in addition to providing method syntax
 * and not having to repeat the type of self in every method’s signature, is for organization.
 * We’ve put all the things we can do with an instance of a type in one impl block rather than
 * making future users of our code search for capabilities of Rectangle in various places in the
 * library we provide.

 * Note that we can choose to give a method the same name as one of the struct’s fields.
 * For example, we can define a method on Rectangle that is also named width.
 */
fn example_of_width_method_inside_a_rectangle() {
    // fn width was here

    main();

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }
}

/* Checks if one Rectangle can completely contain another Rectangle.
 * Returns `true` if the Rectangle passed as an argument can fit entirely within `self` (the first
 * Rectangle), otherwise returns `false`.
 */
fn methods_with_more_parameters() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn example_for_making_a_square_using_associated_function() {
    let sq = Rectangle::square(3);

    println!("Square {:?}", sq);
}