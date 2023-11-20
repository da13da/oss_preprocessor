use reqwest;
use serde_json;

use crate::entities::github::CompareData;

pub struct GithubClient {
    pub url: String,
    client: reqwest::Client,
}

impl GithubClient {
    pub fn new() -> Self {
        GithubClient {
            url: "https://api.github.com/repos".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_latest_to_current_changes(
        &self,
        owner_name: &str,
        package_name: &str,
        current: &str,
        latest: &str,
    ) -> Result<CompareData, reqwest::Error> {
        let url = format!(
            "{}/{}/{}/compare/{}...{}",
            self.url, owner_name, package_name, latest, current
        );
        println!("fetch {:?}", url);
        let compare_data = self
            .client
            .get(&url)
            .header("User-Agent", "request")
            .send()
            .await?
            .json()
            .await?;

        println!("{:?}", compare_data);
        Ok(compare_data)
    }
}
