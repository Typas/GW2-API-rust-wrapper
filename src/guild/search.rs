use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub fn name(self, name: &str) -> NameBuilder {
        NameBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "?name=" + name,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/search",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NameData {}

impl NameData {}

pub struct NameBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl NameBuilder {
    pub async fn get(self) -> ApiResult<NameData> {
        todo!()
    }
}
