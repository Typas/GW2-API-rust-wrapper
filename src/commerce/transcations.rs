use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

impl Data {}

#[derive(Clone, Copy)]
enum Filled {
    Current,
    History,
}

#[derive(Clone, Copy)]
enum BuySell {
    Buys,
    Sells,
}

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub fn current(self) -> Level1Builder {
        Level1Builder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            time_type: Filled::Current,
        }
    }

    pub fn history(self) -> Level1Builder {
        Level1Builder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            time_type: Filled::History,
        }
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/transcations",
        }
    }
}

#[derive(Clone)]
pub struct Level1Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    time_type: Filled,
}

impl Level1Builder {
    pub fn buys(self) -> Level2Builder {
        Level2Builder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            time_type: self.time_type,
            trans_type: BuySell::Buys,
        }
    }

    pub fn sells(self) -> Level2Builder {
        Level2Builder {
            client: self.client,
            key: self.key,
            version: self.version,
            url: self.url,
            time_type: self.time_type,
            trans_type: BuySell::Sells,
        }
    }
}

#[derive(Clone)]
pub struct Level2Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
    time_type: Filled,
    trans_type: BuySell,
}

impl Level2Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }
}
