pub mod permissions;
pub mod search;
pub mod upgrades;
pub mod gid;

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
    pub fn id(self, gid: &str) -> gid::Builder {
        gid::Builder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url + "/" + gid,
        }
    }

    into_builder!(permissions, permissions::Builder);
    into_builder!(search, search::Builder);
    into_builder!(upgrades, upgrades::Builder);
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/guild".to_string(),
        }
    }
}
