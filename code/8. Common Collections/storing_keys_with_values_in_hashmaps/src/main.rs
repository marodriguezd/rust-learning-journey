use std::collections::HashMap;

fn main() {
    creating_new_hash_map();

    // accessing_values_in_a_hash_map();

    hash_maps_and_ownership();

    updating_a_hash_map();

    // overwriting_a_value();

    // adding_a_key_and_value_only_if_a_key_isnt_present();

    updating_a_value_based_on_the_old_value();
}

fn creating_new_hash_map() -> HashMap<String, u32> {
    // We can create a hash map just declaring it with new
    let mut scores = HashMap::new();

    // Now we can add elements with insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each
    // other, and all of the values must have the same type.

    return scores;  // Added by me for commodity
}

fn accessing_values_in_a_hash_map() {
    let scores = creating_new_hash_map();

    // We can get a value out of the hash map by providing its key to the get method
    let team_name = String::from("Blue");

    // Retrieves the score associated with the specified team name from the scores hash map.
    // If no entry exists for that key, it defaults to a zero value representing an unassigned score.
    let _score = scores.get(&team_name).copied().unwrap_or(0);
    // The "_" is because is completely unused in any near moment.

    // We can iterate over the HashMap the same way that over a Vector
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hash_maps_and_ownership() {
    // In Rust, when using types that implement the Copy trait like i32, values are copied into
    // the HashMap. For owned values such as String, the value will be moved and the HashMap becomes
    // the owner of these values. This distinction is crucial in understanding how data ownership
    // works with Rust's borrowing rules and helps prevent issues like double frees or
    // use-after-free errors.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, using them will do a compiler error!
    // This is because they are Strings.
}

fn updating_a_hash_map() {
    // Each unique key can only have one value, though multiple keys can share the same value.
    // To update a HashMap, you can either:
    // 1. Replace the old value with the new value.
    // 2. Keep the old value and only add the new value if the key doesn't already have a value.
    // 3. Combine the old value with the new value.
}

fn overwriting_a_value() {
    // Inserting the same key with a different value will replace the existing value.
    let mut scores = creating_new_hash_map();

    println!("{scores:?}");  // My personal addition to compare

    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

fn adding_a_key_and_value_only_if_a_key_isnt_present() {
    // Use the entry API to check if a key exists and insert a value if it doesn't.
    // The entry method returns an Entry enum that represents a value that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // The or_insert method on Entry returns a mutable reference to the value if the key exists.
    // If the key doesn't exist, it inserts the new value and returns a mutable reference to it.
    // This approach is cleaner and works well with the borrow checker.

}

fn updating_a_value_based_on_the_old_value() {
    // Use a hash map to count occurrences of each word in a text,
    // inserting 0 for new words and updating counts for existing words.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Split the text into words and iterate over each word
    for word in text.split_whitespace() {
        // Use the entry method to get the entry for the word
        // If the word doesn't exist, insert 0 and get a mutable reference to it
        let count = map.entry(word).or_insert(0);
        // Deference the mutable reference and increment the count
        *count += 1;
        // or_insert returns a mutable reference to the key's value or inserts 0 if not present.
        // We must use *count to dereference the mutable reference and update the value.

    }

    println!("{map:?}");
}