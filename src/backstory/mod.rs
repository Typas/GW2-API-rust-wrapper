pub mod answers;
pub mod questions;

use crate::util::*;
use crate::{ApiClient, ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

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

    into_builder!(answers, answers::Builder);
    into_builder!(questions, questions::Builder);
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/backstory".to_string(),
        }
    }
}
