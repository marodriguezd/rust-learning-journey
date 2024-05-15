fn main() {
    example_of_coins_in_a_match();

    patterns_that_bind_to_values();

    matching_with_option();
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