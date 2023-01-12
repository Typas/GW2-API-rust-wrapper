use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AchievementsGroupsData {}

impl AchievementsGroupsData {}

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<AchievementsGroupsData> {
        todo!()
    }

    pub fn id(self, guid: &str) -> IdBuilder {
        IdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/",
            guid: guid.to_string(),
        }
    }

    pub fn ids(self, guids: Vec<String>) -> MultiIdBuilder {
        MultiIdBuilder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            guids,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/groups",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct IdData {}

impl IdData {}

pub struct IdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    guid: String,
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
    guids: Vec<String>,
}

impl MultiIdBuilder {
    pub async fn get(self) -> ApiResult<Vec<IdData>> {
        todo!()
    }
}
