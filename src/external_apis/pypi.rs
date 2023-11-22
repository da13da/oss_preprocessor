use reqwest;
use serde_json;

use crate::entities::pypi::PyPIPackageDetail;
use crate::external_apis::error::FetchError;

pub struct PypiClient {
    pub url: String,
    client: reqwest::Client,
}

impl PypiClient {
    pub fn new() -> Self {
        PypiClient {
            url: "https://pypi.org/pypi".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_package_detail(
        &self,
        package_name: &str,
    ) -> Result<PyPIPackageDetail, FetchError> {
        let url = format!("{}/{}/json", self.url, package_name);

        let response = self.client.get(&url).send().await?;
        if response.status() != 200 {
            return Err(FetchError::StatusCode(response.status().as_u16()));
        }

        let package_detail = response
            .json()
            .await?;

        Ok(package_detail)
    }
}
