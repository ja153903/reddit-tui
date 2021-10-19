use super::constants::{Frequency, Index};
use super::errors::RedditQueryError;

pub fn convert_index_to_string(index: &Index) -> String {
    match index {
        Index::HOT => String::from("hot"),
        Index::TOP => String::from("top"),
        Index::NEW => String::from("new"),
        Index::RISING => String::from("rising"),
    }
}

pub fn convert_frequency_to_string(frequency: &Frequency) -> String {
    match frequency {
        Frequency::DAY => String::from("day"),
        Frequency::WEEK => String::from("week"),
        Frequency::MONTH => String::from("month"),
        Frequency::YEAR => String::from("year"),
        Frequency::ALLTIME => String::from("all"),
    }
}

pub fn build_subreddit_url_by_index(
    subreddit: &str,
    index: &Index,
) -> Result<String, RedditQueryError> {
    if !subreddit.is_empty() {
        let index_as_string = convert_index_to_string(index);
        Ok(format!(
            "https://www.reddit.com/r/{}/{}.json",
            subreddit, index_as_string
        ))
    } else {
        Err(RedditQueryError::EmptySubredditError(String::from(
            "Subreddit in query was empty",
        )))
    }
}

pub fn add_query_config_options(
    url: &str,
    frequency: &Frequency,
    limit: u32,
) -> Result<String, ()> {
    let frequency_as_string = convert_frequency_to_string(frequency);
    Ok(format!("{}?limit={},t={}", url, limit, frequency_as_string))
}

#[cfg(test)]
mod tests {
    use super::{
        build_subreddit_url_by_index, convert_frequency_to_string, convert_index_to_string,
        Frequency, Index, RedditQueryError,
    };

    #[test]
    fn test_should_create_index_as_string() {
        assert_eq!(convert_index_to_string(&Index::TOP), String::from("top"));
        assert_eq!(convert_index_to_string(&Index::HOT), String::from("hot"));
        assert_eq!(convert_index_to_string(&Index::NEW), String::from("new"));
        assert_eq!(
            convert_index_to_string(&Index::RISING),
            String::from("rising")
        );
    }

    #[test]
    fn test_should_build_subreddit_url_by_index() {
        let subreddit = "manga";
        let index = Index::TOP;

        assert_eq!(
            build_subreddit_url_by_index(subreddit, &index),
            Ok(String::from("https://www.reddit.com/r/manga/top.json"))
        );
    }

    #[test]
    fn test_should_fail_build_subreddit_url_by_index() {
        let subreddit = "";
        let index = Index::TOP;

        let result = build_subreddit_url_by_index(subreddit, &index);

        match result {
            Ok(_) => {}
            Err(s) => assert_eq!(
                s,
                RedditQueryError::EmptySubredditError(String::from("Subreddit in query was empty"))
            ),
        }
    }

    #[test]
    fn test_should_create_frequency_as_string() {
        assert_eq!(
            convert_frequency_to_string(&Frequency::DAY),
            String::from("day")
        );
        assert_eq!(
            convert_frequency_to_string(&Frequency::MONTH),
            String::from("month")
        );
        assert_eq!(
            convert_frequency_to_string(&Frequency::YEAR),
            String::from("year")
        );
        assert_eq!(
            convert_frequency_to_string(&Frequency::WEEK),
            String::from("week")
        );
        assert_eq!(
            convert_frequency_to_string(&Frequency::ALLTIME),
            String::from("all")
        );
    }
}
