use std::net::IpAddr;

fn main() {
    // cases_when_you_have_more_information_than_compiler();

    creating_custom_types_for_validation();
}

fn cases_when_you_have_more_information_than_compiler() {
    /// It's appropriate to call unwrap or expect when you have logic ensuring the Result will
    /// have an Ok value, even if the compiler doesn't understand this logic. You still need to
    /// handle the Result because the operation could fail in general. If manual inspection
    /// guarantees no Err variant, using unwrap or expect is acceptable. Documenting the reason in
    /// the expect text is recommended.
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

fn creating_custom_types_for_validation() {
    // Reference of the guessing game done during the course, with few recommendations.
    loop {  // This is insecure
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
        }
    }

    // This way is much better having a custom type with a contract

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}