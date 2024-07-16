/// The `Summary` trait defines a shared behavior for summarizing the content of a type.
/// Any type that implements this trait must define the `summarize` method.
pub trait Summary {
    // V1
    /*
    /// Returns a summary of the content.
    ///
    /// Each type implementing this trait must provide its own custom implementation of the method.
    /// The compiler ensures that any type with the `Summary` trait has the `summarize` method
    /// defined with this exact signature.
    fn summarize(&self) -> String;
    */

    // V2
    /// Returns a summary of the content.
    ///
    /// This method has a default implementation that returns a generic summary string.
    /// Types implementing this trait can override this method to provide a custom summary.
    /// The compiler ensures that any type with the `Summary`trait has the `summarize` method
    /// defined with this exact signature.
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


/// A `NewsArticle` struct representing a news article with a headline, location, author, and content.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// V1
/*
/// Implementing the `Summary` trait for `NewsArticle`.
///
/// The `summarize` method returns a string that summarizes the article using its headline, author, and location.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
*/

// V2
/// Implementing the `Summary` trait for `NewsArticle` with default implementation.
///
/// To use a default implementation to summarize instances of `NewsArticle`.
/// we specify an empty impl block with `impl Summary for NewsArticle {}`.
/// Even though we're no longer defining the summarize method on `NewsArticle` directly,
/// we've provided a default implementation and specified that `NewsArticle` implements
/// the `Summary` trait. As a result, we can still call the summarize method on an instance of `NewsArticle`.
impl Summary for NewsArticle {}

/// A `Tweet` struct representing a tweet with a username, content, and metadata about replies and retweets.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/// Implementing the `Summary` trait for `Tweet`.
///
/// The `summarize` method returns a string that summarizes the tweet using the username and content.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}