use reqwest;

use crate::entities::npm::NpmPackageDetail;
use crate::external_apis::error::FetchError;

pub struct NpmClient {
    pub url: String,
    client: reqwest::Client,
}

impl NpmClient {
    pub fn new() -> Self {
        NpmClient {
            url: "https://registry.npmjs.org".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_package_detail(
        &self,
        package_name: &str,
    ) -> Result<NpmPackageDetail, FetchError> {
        let url = format!("{}/{}", self.url, package_name);

        let response = self.client.get(&url).send().await?;
        if response.status() != 200 {
            return Err(FetchError::StatusCode(response.status().as_u16()));
        }

        let package_detail = response.json().await?;
        println!("{:?}", package_detail);

        Ok(package_detail)
    }
}
