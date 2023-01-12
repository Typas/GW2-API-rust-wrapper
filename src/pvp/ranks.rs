use crate::api::Language;
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
    lang: Option<Language>,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }

    pub fn lang(self, lang: Language) -> Self {
        Self {
            lang: Some(lang),
            ..self
        }
    }

    pub fn id(self, sid: &str) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/",
            lang: self.lang,
            sid: sid.to_string(),
        }
    }

    pub fn ids(self, sids: Vec<String>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            lang: self.lang,
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
            url: source.url + "/cats",
            lang: None,
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
    lang: Option<Language>,
    sid: String,
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
    lang: Option<Language>,
    sids: Vec<String>,
}

impl MultiIdBuilder {
    pub async fn get(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}
