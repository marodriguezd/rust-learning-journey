fn main() {
    example_of_coins_in_a_match();

    patterns_that_bind_to_values();

    matching_with_option();

    catch_all_patterns_and_the_placeholder();
}

fn example_of_coins_in_a_match() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn patterns_that_bind_to_values() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    /* In this example, a friend is collecting all 50 state quarters.
     * As we sort our change by coin type, we print the state for each Coin::Quarter.
     * The match expression binds the state variable to the state value in Coin::Quarter.
     * For instance, calling value_in_cents(Coin::Quarter(UsState::Alaska)) will match
     * Coin::Quarter(state), binding state to UsState::Alaska, and printing the state.
     */

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn matching_with_option() {
    /* This function demonstrates handling Option<T> with match.
     * It takes an Option<i32> and adds 1 to the value if it exists.
     * If there's no value (None), it returns None without performing any operation.
     */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn catch_all_patterns_and_the_placeholder() {
    /* This function demonstrates using match with enums to handle specific values with special
     * actions. In this game example, rolling a 3 gives the player a fancy hat, rolling a 7 removes
     * a fancy hat, and for all other values, the player moves that number of spaces on the game
     * board.
     */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The catch-all pattern (_) ensures match is exhaustive and must be the last arm.
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // Updated game rule: roll again if you get anything other than a 3 or 7 using _ instead of
    // a variable.
    fn dice_rerolled() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        // add_fancy_hat() and remove_fancy_hat() taken from above
        fn reroll() {}
    } // This example is exhaustive as the last arm ignores all other values.

    // Final rule: if you roll anything other than a 3 or 7, nothing happens using the unit
    // value ().
    fn only_dice_3_or_7() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }

        // add_fancy_hat() and remove_fancy_hat() taken from above

    }  // Here, we’re telling Rust explicitly that we aren’t going to use any other value that
       // doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this
       // case.
}