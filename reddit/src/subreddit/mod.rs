use reqwest::Client;

mod constants;
mod errors;
mod utils;

use constants::{Frequency, Index};
use errors::RedditQueryError;
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
        index: &Index,
        frequency: &Frequency,
        limit: u32,
    ) -> Result<String, RedditQueryError> {
        let url: String =
            build_subreddit_url_by_index(subreddit, index).unwrap_or(String::from(""));
        let url_with_options: String =
            add_query_config_options(url.as_str(), limit, frequency).unwrap_or(String::from(""));

        if url_with_options.is_empty() {
            return Err(RedditQueryError::MalformedUrlError(String::from(
                "Unable to create URL for query",
            )));
        }

        // TODO: Need to make sure we create the proper
        // response object for this
        Ok(self.client.get(url_with_options).send().await?);
    }
}
