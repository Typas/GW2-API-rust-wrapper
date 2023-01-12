use crate::api::Language;
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
    lang: Option<Language>,
    page: Option<u32>,
    page_size: Option<u32>,
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

    pub fn page(self, page: u32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    pub fn page_size(self, page_size: u32) -> Self {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    pub fn id(self, id: u32) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/",
            lang: self.lang,
            page: self.page,
            page_size: self.page_size,
            id,
        }
    }

    pub fn ids(self, ids: Vec<u32>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            lang: self.lang,
            page: self.page,
            page_size: self.page_size,
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
            url: source.url + "/skins",
            lang: None,
            page: None,
            page_size: None,
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
    page: Option<u32>,
    page_size: Option<u32>,
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
    lang: Option<Language>,
    page: Option<u32>,
    page_size: Option<u32>,
    ids: Vec<u32>,
}

impl MultiIdBuilder {
    pub async fn get(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}

// TODO: there's parameters `lang`, `page`, `page_size` should be implemented in query.
