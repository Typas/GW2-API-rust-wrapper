pub mod amulets;
pub mod games;
pub mod ranks;
pub mod seasons;
pub mod standings;
pub mod stats;

use crate::util::*;
use crate::{ApiResult, SchemaVersion, ApiClient};
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

    into_builder!(seasons, seasons::Builder);
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/pvp".to_string(),
        }
    }
}
