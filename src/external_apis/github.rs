use reqwest;
use serde_json;

use crate::entities::github::{CompareData, Tag};
use crate::external_apis::error::FetchError;

pub struct GithubClient {
    pub url: String,
    personal_token: String,
    client: reqwest::Client,

}

impl GithubClient {
    pub fn new(token: String) -> Self {
        GithubClient {
            url: "https://api.github.com/repos".to_string(),
            personal_token: token,
            client: reqwest::Client::new(),
        }
    }

    async fn send(&self, url: String) -> Result<reqwest::Response, FetchError>{
        let response = self
            .client
            .get(&url)
            .header("User-Agent", "request")
            .header("Authorization", format!("Bearer {}", &self.personal_token))
            .send()
            .await?;

        if response.status() != 200 {
            return Err(FetchError::StatusCode(response.status().as_u16()));
        }

        Ok(response)
    }

    pub async fn fetch_tags(
        &self,
        owner_name: &str,
        package_name: &str,
    ) -> Result<Vec<Tag>, FetchError> {
        let url = format!(
            "{}/{}/{}/tags",
            self.url, owner_name, package_name
        );

        let tags = self
            .send(url)
            .await?
            .json::<Vec<Tag>>()
            .await?;

        Ok(tags)
    }

    pub async fn fetch_latest_to_current_changes(
        &self,
        owner_name: &str,
        package_name: &str,
        base: &str,
        head: &str,
    ) -> Result<CompareData, FetchError> {
        let url = format!(
            "{}/{}/{}/compare/{}...{}",
            self.url, owner_name, package_name, base, head
        );

        let compare_data = self
            .send(url)
            .await?
            .json()
            .await?;

        Ok(compare_data)
    }
}
