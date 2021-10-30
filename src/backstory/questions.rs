use crate::util::*;
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
    pub async fn build(self) -> ApiResult<Data> {
        todo!()
    }

    pub fn id(self, sid: String) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/",
            sid,
        }
    }

    pub fn ids(self, sids: Vec<String>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "?ids=",
            sids,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/questions",
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
    sid: String,
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
    sids: Vec<String>,
}

impl MultiIdBuilder {
    pub async fn build(self) -> ApiResult<IdData> {
        todo!()
    }
}
