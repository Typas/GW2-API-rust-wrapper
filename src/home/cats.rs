use crate::util::*;
use crate::{SchemaVersion, ApiResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

impl Data {}

#[derive(Clone)]
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
            url: self.url,
            ids,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/cats",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IdData {}

impl IdData {}

pub struct IdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    id: u32,
}

impl IdBuilder {
    pub async fn get(self) -> ApiResult<IdData> {
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
    pub async fn get(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}
