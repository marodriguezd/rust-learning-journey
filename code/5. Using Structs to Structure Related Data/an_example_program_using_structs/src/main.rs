fn main() {
    rectangle();

    rectangle_with_tuples();

    rectangle_with_structs();

    adding_useful_functionality_with_derived_traits();

    dbg_macro_examples();
}

/* This code calculates the area of a rectangle using separate width and height parameters,
 * but it lacks clarity because it doesn't explicitly show that the parameters are related.
 * Using a tuple to group these dimensions could enhance readability and manageability,
 * as suggested previously in the discussion on tuple types. This approach would
 * consolidate width and height into a single parameter, making the relationship between them clearer.
 */

fn rectangle() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Here's the way we said before but...
/* This version of the program uses tuples to structure the rectangle dimensions, improving
 * simplicity by reducing the number of arguments to one. However, tuples do not name their elements,
 * which makes accessing these elements by index less clear and prone to errors.
 * For example, the order of width and height is crucial when drawing the rectangle on screen,
 * but their indices (0 for width and 1 for height) do not convey meaning, increasing the risk
 * of errors, especially for others who might use or maintain this code.
 */

fn rectangle_with_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuples(rect1)
    );
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Now we're going to refactor with Structs

struct Rectangle {
    width: u32,
    height: u32
}

fn rectangle_with_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_structs(&rect1)
    );
}

fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn adding_useful_functionality_with_derived_traits() {
    // In order to use Rectangle in a debugging way we need to add on it the debugging capabilities.
    // This is because struct don't have a proper "Display" option in println!.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    main();

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        // Putting the specifier :? inside the curly brackets tells println! we want to use an
        // output format called Debug.
        println!("rect1 is {:?}", rect1);

        // We have another way to display this, and it's using: {:#?} instead of {:?}.
        println!("rect1 is {:#?}", rect1);
    }
}

/* Another way to print out a value using the Debug format is to use the dbg! macro, which
 * takes ownership of an expression (as opposed to println!, which takes a reference), prints
 * the file and line number of where that dbg! macro call occurs in your code along with the
 * resultant value of that expression, and returns ownership of the value.
 */

fn dbg_macro_examples() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    main();

    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of
    // the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. Here’s what the output of this example looks like

    fn main() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}
