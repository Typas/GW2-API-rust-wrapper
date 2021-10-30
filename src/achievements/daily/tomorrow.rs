use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub enum Data {
    Pre20190516(DataPre20190516Schema),
    Post20190516(DataPost20190516Schema),
}

impl Data {}

#[derive(Deserialize, Serialize)]
pub struct DataPre20190516Schema {}

trait_try_from_jsonvalue!(DataPre20190516Schema);

#[derive(Deserialize, Serialize)]
pub struct DataPost20190516Schema {}

trait_try_from_jsonvalue!(DataPost20190516Schema);

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
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/tomorrow",
        }
    }
}
