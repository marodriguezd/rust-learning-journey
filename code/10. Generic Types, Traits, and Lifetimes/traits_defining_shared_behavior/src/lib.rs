/// The `Summary` trait defines a shared behavior for summarizing the content of a type.
/// Any type that implements this trait must define the `summarize` method.
pub trait Summary {
    /// Returns a summary of the content.
    ///
    /// Each type implementing this trait must provide its own custom implementation of the method.
    /// The compiler ensures that any type with the `Summary` trait has the `summarize` method
    /// defined with this exact signature.
    fn summarize(&self) -> String;
}


/// A `NewsArticle` struct representing a news article with a headline, location, author, and content.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Implementing the `Summary` trait for `NewsArticle`.
///
/// The `summarize` method returns a string that summarizes the article using its headline, author, and location.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

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