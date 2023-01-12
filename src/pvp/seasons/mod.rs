pub mod leaderboards;

use crate::api::{NotAuthenticatedError, Language};
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

    pub async fn id(self, id: &str) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/" + id,
            lang: self.lang,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/seasons",
            lang: None,
        }
    }
}

pub struct IdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    lang: Option<Language>,
}

impl IdBuilder {
    into_builder!(leaderboards, leaderboards::Builder);
}
