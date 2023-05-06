use crate::{util::*, Language};
use crate::{ApiResult, SchemaVersion};
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

    pub fn ids(self, sids: Vec<String>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            sids,
            lang: None,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/permissions",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct IdData {}

impl IdData {}

pub struct MultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    sids: Vec<String>,
    lang: Option<Language>,
}

impl MultiIdBuilder {
    pub async fn get(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}
