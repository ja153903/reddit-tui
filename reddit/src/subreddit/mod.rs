use reqwest::Client;

mod constants;
mod errors;
mod utils;

use utils::{add_query_config_options, build_subreddit_url_by_index};

pub struct RedditSubreddit {
    pub client: Client,
}

impl RedditSubreddit {
    pub fn new(&self, client: &Client) -> Self {
        Self { client }
    }

    pub async fn query(
        &self,
        subreddit: &str,
        index: &str,
        frequency: &str,
        limit: u32,
    ) -> Result<String, ()> {
        let url: String = build_subreddit_url_by_index(subreddit, index);
        let url_with_options: Option<String> = if !frequency.is_empty() || !frequency.is_empty() {
            Some(add_query_config_options(url, frequency, limit))
        } else {
            None
        };

        let result = if let Some(url) = url_with_options {
            self.client.get(url_with_options).send().await?
        } else {
            None
        };

        println!("{:?}", result);

        Ok(String::from("Something"))
    }
}
