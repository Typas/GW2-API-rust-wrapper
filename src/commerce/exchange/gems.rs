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
    quantity: Option<u32>,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }

    pub fn quantity(self, quantity: u32) -> Self {
        Self {
            quantity: Some(quantity),
            ..self
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/gems",
            quantity: None,
        }
    }
}
