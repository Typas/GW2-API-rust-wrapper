pub mod categories;
pub mod daily;
pub mod groups;

use crate::util::*;
use crate::{ApiClient, ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn build(self) -> ApiResult<Data> {
        let req = request_common_build(&self.client, &self.key, &self.version, &self.url);

        let data = req.send().await?.json().await?;
        Ok(data)
    }

    into_builder!(daily, daily::Builder);
    into_builder!(categories, categories::Builder);
    into_builder!(groups, groups::Builder);

    pub fn id(self, id: u32) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/",
            id,
        }
    }

    pub fn ids(self, ids: Vec<u32>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "?ids=",
            ids,
        }
    }
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/achievements".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct IdData {}

pub struct IdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    id: u32,
}

impl IdBuilder {
    pub async fn build(self) -> ApiResult<IdData> {
        todo!()
    }
}

pub struct MultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    ids: Vec<u32>,
}

impl MultiIdBuilder {
    pub async fn build(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}
