pub fn main() {
    // Test the conversion with the word "apple"
    let word = "apple";
    let pig_latin_word = convert_to_pig_latin(word.trim());
    println!("{}", pig_latin_word);

    // Test the conversion with the word "spanish"
    let word = "spanish";
    let pig_latin_word = convert_to_pig_latin(word.trim());
    println!("{}", pig_latin_word);
}

/// Main function to convert a given word to Pig Latin.
///
/// # Arguments
///
/// * 'word' - A string slice that holds the word to be converted.
///
/// # Returns
///
/// * A 'String' containing the word converted to Pig Latin.
fn convert_to_pig_latin(word: &str) -> String {
    // Get the first character of the word to check if it's a vowel
    let first_char = word.chars().next().unwrap();
    if is_vowel(first_char) {
        // If the first character is a vowel, append "-hay" to the word
        format!("{}-hay", word)
    } else {
        // If the first character is a consonant, move it to the end and append "-ay"
        let mut chars = word.chars();
        let first_consonant = chars.next().unwrap();
        let rest_of_word: String = chars.collect();
        format!("{}-{}ay", rest_of_word, first_consonant)
    }
}

/// Function to determine if a given character is a vowel.
///
/// # Arguments
///
/// * 'c' - A character to be checked.
///
/// # Returns
///
/// * 'true' if the character is a vowel, 'false' otherwise.
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}