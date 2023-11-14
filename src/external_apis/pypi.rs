use reqwest;
use serde_json;

use crate::entities::pypi::PyPIPackageResponse;

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

    pub async fn get_package_details(
        &self,
        package_name: &str,
    ) -> Result<PyPIPackageResponse, reqwest::Error> {
        let url = format!("{}/{}/json", self.url, package_name);
        let response = self.client.get(&url).send().await?;

        response.json().await
    }
}
