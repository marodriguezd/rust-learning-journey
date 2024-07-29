// This is aggregator in the Rust book
use traits_defining_shared_behavior::{NewsArticle, Summary, Tweet};
use std::fmt::Display;

/// The `notify` function accepts any type that implements the `Summary` trait.
/// It prints a breaking news message with the summary of the item passed to it.
///
/// # Arguments
///
/// * `item` - A reference to an item that implements the `Summary` trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// The `notify_with_trait_bound` function uses trait bound syntax to accept any type that implements
/// the `Summary` trait.
/// It prints a breaking news message with the summary of the item passed to it.
///
/// # Arguments
///
/// * `item` - A reference to an item that implements the `Summary` trait.
pub fn notify_with_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// The `notify_with_multiple_bounds` function accepts any type that implements both `Summary` and
/// `Display` traits.
/// It prints a breaking news message with the summary and formatted display of the item passed to it.
///
/// # Arguments
///
/// * `item` - A reference to an item that implements both the `Summary` and `Display` traits.
pub fn notify_with_multiple_bounds(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
    println!("Formatted item: {}", item);
}

/// The `notify_with_multiple_trait_bounds` function uses trait bound syntax to accept any type that
/// implements both `Summary` and `Display` traits.
/// It prints a breaking news message with the summary and formatted display of the item passed to it.
///
/// # Arguments
///
/// * `item` - A reference to an item that implements both the `Summary` and `Display` traits.
pub fn notify_with_multiple_trait_bounds<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Formatted item: {}", item);
}

/// The `some_function` uses a where clause to simplify the function signature when
/// multiple trait bounds are needed.
///
/// # Arguments
///
/// * `t` - A reference to an item that implements both `Display` and `Clone` traits.
/// * `u` - A reference to an item that implements both `Clone` and `Debug` traits.
///
/// # Returns
///
/// * Returns an integer.
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    println!("t: {}, u: {}", t, u);
    42
}
fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // Display the summary of the tweet
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    // Use the notify functions to print the summary of the tweet
    notify(&tweet);
    notify_with_trait_bound(&tweet);

    // println!("New article available! {}", article.summarize());
    // BROKEN IN LIB
}