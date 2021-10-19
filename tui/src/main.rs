use reddit::client::RedditClient;

fn main() {
    let reddit_connection = RedditClient::new();
    println!("{}", reddit_connection.message);
}
