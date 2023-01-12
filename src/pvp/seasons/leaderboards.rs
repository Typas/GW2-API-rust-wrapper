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
}

impl From<super::IdBuilder> for Builder {
    fn from(source: super::IdBuilder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/leaderboards",
            lang: None,
        }
    }
}
