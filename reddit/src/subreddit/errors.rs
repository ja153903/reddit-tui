#[derive(Debug, PartialEq)]
pub enum RedditQueryError {
    EmptySubredditError(String),
    MalformedUrlError(String),
}
