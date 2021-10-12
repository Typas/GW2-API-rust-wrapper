use std::{fmt, sync::Arc};

use crate::util::request_common_build;
use chrono::Utc;

use super::ApiResult;
use reqwest::{Client, ClientBuilder};
use serde_json::Value as json_value;

#[allow(dead_code)]
pub struct ApiClient {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

#[allow(dead_code)]
pub struct ApiClientBuilder {
    client: Client,
    key: Option<String>,
    version: SchemaVersion,
}

impl ApiClient {
    /// create a api client without any authentication
    pub fn new() -> ApiResult<Self> {
        Ok(Self::config()?.build())
    }

    /// configure api client with api key and schema version
    pub fn config() -> ApiResult<ApiClientBuilder> {
        ApiClientBuilder::new()
    }

    /// get some data without any limit, need to parse data by yourself
    pub async fn free_style(&self, param: &str) -> ApiResult<json_value> {
        let url = String::from("https://api.guildwars2.com/v2/") + param;

        let req = self.client.get(&url);
        let req = request_common_build(req, &self.key, &self.version);

        let json: json_value = req.send().await?.json().await?;
        Ok(json)
    }

    /// build account request
    pub fn account(&self) -> super::account::AccountBuilder {
        super::account::AccountBuilder {
            client: self.client.clone(),
            key: self.key.clone(),
            version: self.version.clone(),
        }
    }
}

impl ApiClientBuilder {
    /// create a default setting
    pub fn new() -> ApiResult<Self> {
        let client = ClientBuilder::new().https_only(true).build()?;

        let item = ApiClientBuilder {
            key: None,
            client,
            version: SchemaVersion::Latest,
        };

        Ok(item)
    }

    /// change api key of client
    pub fn key(self, key: &str) -> Self {
        let k = key.to_owned();
        ApiClientBuilder {
            key: Some(k),
            client: self.client,
            version: self.version,
        }
    }

    /// change schema version of client
    pub fn schema(self, version: SchemaVersion) -> Self {
        ApiClientBuilder {
            key: self.key,
            client: self.client,
            version,
        }
    }

    /// build a api client
    pub fn build(self) -> ApiClient {
        ApiClient {
            key: Arc::new(self.key),
            client: self.client,
            version: Arc::new(self.version),
        }
    }
}

pub enum SchemaVersion {
    Default,
    Time(chrono::DateTime<Utc>),
    Latest,
}

#[derive(Debug)]
pub struct NotAuthenticatedError;

impl fmt::Display for NotAuthenticatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Accessing content that need authentication without api key"
        )
    }
}

impl std::error::Error for NotAuthenticatedError {}
