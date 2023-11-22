use reqwest;
use serde::de::Error;
use serde_json;

use crate::entities::github::{CompareData, Release, Tag};

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

    pub async fn fetch_release_info(
        &self,
        owner_name: &str,
        package_name: &str,
        version: &str
    ) -> Result<Release, reqwest::Error> {
        let url = format!(
            "{}/{}/{}/releases/{}",
            self.url, owner_name, package_name, version,
        );

        println!("fetch: {:?}", url);
        let release = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.personal_token))
            .send()
            .await?
            .json()
            .await?;

        println!("release: {:?}", release);
        Ok(release)
    }

    pub async fn fetch_tags(
        &self,
        owner_name: &str,
        package_name: &str,
    ) -> Result<Vec<Tag>, reqwest::Error> {
        let url = format!(
            "{}/{}/{}/tags",
            self.url, owner_name, package_name
        );

        let tags = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.personal_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(tags)
    }

    pub async fn fetch_latest_to_current_changes(
        &self,
        owner_name: &str,
        package_name: &str,
        base: &str,
        head: &str,
    ) -> Result<CompareData, reqwest::Error> {
        let url = format!(
            "{}/{}/{}/compare/{}...{}",
            self.url, owner_name, package_name, base, head
        );

        let compare_data = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.personal_token))
            .send()
            .await?
            .json()
            .await?;

        println!("{:?}", compare_data);
        Ok(compare_data)
    }
}
