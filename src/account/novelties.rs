use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

impl Data {}

pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/novelties",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ApiClient;

    fn setup() -> ApiClient {
        ApiClient::new().unwrap()
    }

    // need environment variable GW2_API_KEY
    fn setup_auth() -> ApiClient {
        let key = std::env::var("GW2_API_KEY").unwrap();
        ApiClient::config().unwrap().key(&key).build()
    }

    #[tokio::test]
    #[should_panic]
    async fn build_no_auth() {
        let client = setup();
        let _t: super::Data = client.account().novelties().build().await.unwrap();
    }

    #[tokio::test]
    async fn build_with_auth() {
        let client = setup_auth();
        let _t: super::Data = client.account().novelties().build().await.unwrap();
    }
}
