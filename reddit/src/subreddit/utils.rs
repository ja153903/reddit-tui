pub fn build_subreddit_url_by_index(subreddit: &str, index: &str) -> String {
    format!("https://www.reddit.com/r/{}/{}.json", subreddit, index)
}

pub fn add_query_config_options(url: &str, frequency: &str, limit: u32) -> String {
    format!("{}?limit={},t={}", url, limit, frequency)
}
