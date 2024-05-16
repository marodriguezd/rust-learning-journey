enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    handle_config_max();
}

fn handle_config_max() {
    // Using match to handle Some variant in Option<u8> and print the value.
    let config_max = Some(3u8);
    match config_max {  // Only execute code if the value is Some.
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if let to achieve the same result with less boilerplate.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /* Using if let means less typing, less indentation, and less boilerplate code. However, you
     * lose the exhaustive checking that match enforces. Choosing between match and if let depends
     * on what you’re doing in your particular situation and whether gaining conciseness is an
     * appropriate trade-off for losing exhaustive checking.
     */

    // Use if let with else to handle specific and catch-all cases, like in match.
    fn example_recovering_coins(coin: Coin) {
        let mut count = 0;
        match coin {  // This is using match
            Coin::Quarter(state) => println!("State quarter from {:?}", state),
            _ => count += 1,
        }

        let mut count = 0;
        if let Coin::Quarter(state) = coin {  // Using if let
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }
}
