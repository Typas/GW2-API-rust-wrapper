use crate::util::request_common_build;

use super::ApiResult;
use reqwest::Client;
use serde_json::Value as JsonValue;
use std::{fmt, sync::Arc};

#[allow(dead_code)]
pub struct ApiClient {
    pub(crate) key: Arc<Option<String>>,
    pub(crate) client: Client,
    pub(crate) version: Arc<SchemaVersion>,
}

#[allow(dead_code)]
pub struct ApiClientBuilder {
    key: Arc<Option<String>>,
    client: Client,
    version: Arc<SchemaVersion>,
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

    /// get API date by selecting endpoint
    /// remember to use right type to get the data, or it would panic
    pub async fn free_style(&self, path: &str) -> ApiResult<JsonValue> {
        let url = String::from("https://api.guildwars2.com/v2/") + path;

        // XXX: I want to combine these to one statement
        let req = request_common_build(&self.client, &self.key, &self.version, &url);

        let data = req.send().await?.json().await?;

        Ok(data)
    }

    pub fn account(&self) -> crate::account::Builder {
        self.into()
    }

    pub fn achievements(&self) -> crate::achievements::Builder {
        self.into()
    }

    pub fn backstory(&self) -> crate::backstory::Builder {
        self.into()
    }

    pub fn commerce(&self) -> crate::commerce::Builder {
        self.into()
    }

    pub fn guild(&self) -> crate::guild::Builder {
        self.into()
    }
}

impl ApiClientBuilder {
    /// create a default setting
    pub fn new() -> ApiResult<Self> {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;

        let item = ApiClientBuilder {
            key: Arc::new(None),
            client,
            version: Arc::new(SchemaVersion::Latest),
        };

        Ok(item)
    }

    /// change api key of client
    pub fn key(self, key: &str) -> Self {
        let k = key.to_owned();
        ApiClientBuilder {
            key: Arc::new(Some(k)),
            client: self.client,
            version: self.version,
        }
    }

    /// change schema version of client
    pub fn schema(self, version: SchemaVersion) -> Self {
        ApiClientBuilder {
            key: self.key,
            client: self.client,
            version: Arc::new(version),
        }
    }

    /// build a api client
    pub fn build(self) -> ApiClient {
        ApiClient {
            key: self.key,
            client: self.client,
            version: self.version,
        }
    }
}

#[derive(Clone)]
pub enum SchemaVersion {
    Default,
    Time(chrono::NaiveDateTime),
    Latest,
}

#[derive(Debug)]
pub struct NotAuthenticatedError;

impl fmt::Display for NotAuthenticatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API key is invalid, or void")
    }
}

impl std::error::Error for NotAuthenticatedError {}
