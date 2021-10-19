use reqwest::Client;

pub struct RedditClient {
    pub client: Client,
    pub message: &'static str,
}

impl RedditClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            message: "Client has been created",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RedditClient;

    #[test]
    fn test_get_reddit_client() {
        let client = RedditClient::new();

        assert_eq!(client.message, "Client has been created");
    }
}
